const version = "v1";
const assets = [
    "./",
    "./index.html",
    "./js/db_query_worker.js",
    "./js/db_sync_worker.js",
    "./js/global.js",
    "./js/sqlite3-opfs-async-proxy.js",
    "./js/sqlite3.mjs",
    "./js/sqlite3.wasm",
];

self.addEventListener('install', (e) => {
    // Do precache assets
    e.waitUntil(
	caches
	    .open(version)
	    .then((cache) => {
		cache.addAll(assets);
	    })
	    .then(() => self.skipWaiting())
    );
});

self.addEventListener('activate', (e) => {
    // Delete old versions of the cache
    e.waitUntil(
	caches.keys().then((keys) => {
	    return Promise.all(
		keys.filter((key) => key != version).map((name) => caches.delete(name))
	    );
	})
    );
});

self.addEventListener('fetch', (e) => {
    if (e.request.method !== "GET") {
	return;
    }
    
    const isOnline = self.navigator.onLine;

    const url = new URL(e.request.url);

    if (isOnline) {
	e.respondWith(staleWhileRevalidate(e));
    } else {
	e.respondWith(cacheOnly(e));
    }
});

function cacheOnly(e) {
    return caches.match(e.request);
}

function staleWhileRevalidate(ev) {
    return caches.match(ev.request).then((cacheResponse) => {
	let fetchResponse = fetch(ev.request).then((response) => {
	    if (response.ok) {
		return caches.open(version).then((cache) => {
		    cache.put(ev.request, response.clone());
		    return response;
		});		
	    }

	    return cacheResponse;
	});
	return cacheResponse || fetchResponse;
    });
}

function networkRevalidateAndCache(ev) {
  return fetch(ev.request, { mode: 'cors', credentials: 'omit' }).then(
    (fetchResponse) => {
      if (fetchResponse.ok) {
        return caches.open(version).then((cache) => {
          cache.put(ev.request, fetchResponse.clone());
          return fetchResponse;
        });
      } else {
        return caches.match(ev.request);
      }
    }
  );
}


