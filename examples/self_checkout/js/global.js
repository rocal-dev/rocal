function execSQL(db, query) {
    return new Promise((resolve, reject) => {
	const worker = new Worker("./js/db_query_worker.js", { type: 'module' });
	worker.postMessage({ db: db, query: query });

	worker.onmessage = function (message) {
	    resolve(message.data);
	    worker.terminate();
	};

	worker.onerror = function (err) {
	    reject(err);
	    worker.terminate();	    
	};
    });
}
