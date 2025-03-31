import sqlite3InitModule from './sqlite3.mjs';

self.onmessage = async function (message) {
    const { app_id, directory_name, file_name, endpoint, force } = message.data;
    
    self.sqlite3InitModule().then((sqlite3) => {
	if (sqlite3.capi.sqlite3_vfs_find("opfs")) {
	    const db = new sqlite3.oo1.OpfsDb(`${directory_name}/${file_name}`, "ct");
	    const query = "select id, password from sync_connections order by created_at asc limit 1;";
	    const result = db.exec(query, { rowMode: 'array' });

	    if (0 < result.length && 1 < result[0].length) {
		const user_id = result[0][0];
		const password = result[0][1];

		if (force !== "none") {
		    sync(app_id, user_id, password, directory_name, file_name, endpoint, force);
		    setInterval(sync, 30000, app_id, user_id, password, directory_name, file_name, endpoint, null);
		} else {
		    setInterval(sync, 30000, app_id, user_id, password, directory_name, file_name, endpoint, force);
		}
	    }
	} else {
	    console.error("OPFS not available because of your browser capability.");
	}
    });
};

async function sync(app_id, user_id, password, directory_name, file_name, endpoint, force) {
    console.log('Syncing..');
    
    try {
	const file = await getFile(directory_name, file_name);

	const last_modified = file === null || force === "remote" ? 0 : Math.floor(file.lastModified / 1000);

	const response = await fetch(endpoint, {
	    method: "POST",
	    headers: { "Content-Type": "application/json" },
	    body: JSON.stringify({ app_id, user_id, password, file_name, unix_timestamp: last_modified }),
	    credentials: "include"
	});
	
	if (!response.ok) {
	    console.error("Sync API is not working now");
	    return;
	}

	const json = await response.json();

	const obj = JSON.parse(json);
	
	if (obj.presigned_url === null || obj.last_modified_url === null || obj.action === null) {
	    console.log("No need to sync your database");
	    return;
	}

	if (obj.action === "get_object") {
	    const res = await fetch(obj.presigned_url, { method: "GET" });
	    
	    const fileHandler = await getFileHandler(directory_name, file_name, file === null);

	    if (fileHandler === null) {
		return;
	    }
	    
	    const fileAccessHandler = await fileHandler.createSyncAccessHandle();

	    const arrayBuffer = await res.arrayBuffer();
	    const uint8Array = new Uint8Array(arrayBuffer);

	    fileAccessHandler.write(uint8Array, { at: 0 });
	    fileAccessHandler.flush();
	    
	    fileAccessHandler.close();
	} else if (obj.action === "put_object") {
	    const arrayBuffer = await file.arrayBuffer();	    
	    await Promise.all([
		fetch(obj.presigned_url, { method: "PUT", headers: { "Content-Type": "application/vnd.sqlite3" }, body: arrayBuffer }),
		fetch(obj.last_modified_url, { method: "PUT", headers: { "Content-Type": "text/plain" }, body: new File([last_modified], "LASTMODIFIED", { type: "text/plain" }) })
	    ]);
	}

	console.log('Synced');
    } catch (err) {
	console.error(err.message);
    }    
}

async function getFile(directory_name, file_name) {
    try {
	const fileHandler = await getFileHandler(directory_name, file_name);
	return await fileHandler.getFile();
    } catch (err) {
	console.error(err.message, ": Cannot find the file");
	return null;
    }
}

async function getFileHandler(directory_name, file_name, create = false) {
    try {
	const root = await navigator.storage.getDirectory();
	const dirHandler = await root.getDirectoryHandle(directory_name, { create: create });
	return await dirHandler.getFileHandle(file_name, { create: create });
    } catch (err) {
	console.error(err.message, ": Cannot get file handler");
	return null;
    }    
}
