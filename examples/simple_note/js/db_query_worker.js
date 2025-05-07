import sqlite3InitModule from './sqlite3.mjs';

const dbCache = Object.create(null);

self.onmessage = function (message) {
    const db_name = message.data.db;

    self.sqlite3InitModule().then((sqlite3) => {
	if (sqlite3.capi.sqlite3_vfs_find("opfs")) {
	    const db = dbCache[db_name] ??= new sqlite3.oo1.OpfsDb(db_name, "ct");
	    if (!!message.data.query) {
		self.postMessage(db.exec(message.data.query, { bind: message.data.bindings, rowMode: 'object' }));
	    }
	} else {
	    console.error("OPFS not available because of your browser capability.");
	}
    });
};
