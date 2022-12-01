var cacheName = 'timeline_rs';
var filesToCache = [
  './',
  './index.html',
  './timeline_rs.js',
  './timeline_rs_bg.wasm',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    console.log("wait install"),
    resetCacheForUpdate(),
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});

async function deleteCaches() {
  console.log("delete caches")
  try {
    const keys = await window.caches.keys();
    await Promise.all(keys.map(key => caches.delete(key)));
  } catch (err) {
    console.log('deleteCache err: ', err);
  }
}

// run this function on your app load
function resetCacheForUpdate() {
  console.log("reset cache for update")
  if (!localStorage.getItem('cacheReset')) {
    deleteCaches()
      .then(_ => {
        localStorage.setItem('cacheReset', 'yes');
      })
  }
}