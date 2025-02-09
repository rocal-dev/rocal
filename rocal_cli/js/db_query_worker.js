import sqlite3InitModule from './sqlite3.mjs';

self.onmessage = function (message) {
    const db_name = message.data.db;

    self.sqlite3InitModule().then((sqlite3) => {
	if (sqlite3.capi.sqlite3_vfs_find("opfs")) {
	    const db = new sqlite3.oo1.OpfsDb(db_name, "ct");
	    self.postMessage(db.exec(message.data.query, { rowMode: 'object' }));
	} else {
	    console.error("OPFS not available because of your browser capability.");
	}
    });
};
