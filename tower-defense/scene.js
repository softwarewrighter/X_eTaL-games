// The 3D view of a scene an X_eTaL program describes. A game's picture
// (its SVG, from []S_HOW) may carry a scene as JSON in
// <metadata id="scene">: a camera, tiles, pieces (a model, where it
// stands, the way it faces, the clip it plays and how far into it) and
// shots. The page (microscope) hands that text to update() with a canvas
// behind the picture; this draws exactly that scene with three.js and
// nothing else: every number comes from the program. update() answers
// whether it drew every piece (their models loaded), so the page hides
// the picture's flat board and pictures only then.
import * as THREE from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";

const loader = new GLTFLoader();
const templates = new Map(); // model URL -> { gltf } once loaded (null while loading)
const views = new WeakMap(); // canvas -> its renderer, scene, camera, pools

function model(url) {
  if (!templates.has(url)) {
    templates.set(url, null);
    loader.load(url, (gltf) => templates.set(url, gltf), undefined, () => templates.delete(url));
  }
  return templates.get(url);
}

function viewOf(canvas) {
  let v = views.get(canvas);
  if (v) return v;
  const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
  renderer.setPixelRatio(window.devicePixelRatio || 1);
  const scene = new THREE.Scene();
  scene.add(new THREE.HemisphereLight(0xffffff, 0x3a4a3a, 1.6));
  const sun = new THREE.DirectionalLight(0xffffff, 1.8);
  sun.position.set(-4, 10, 6);
  scene.add(sun);
  const camera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0.1, 1000);
  v = { renderer, scene, camera, tiles: null, tilesKey: "", pools: new Map(), shots: [] };
  views.set(canvas, v);
  return v;
}

// The tiles: a quad each, a little inset over a dark ground so the grid
// shows, colored by the scene's colors (road or grass).
function tiles(v, t) {
  const key = `${t.cols}:${t.road}:${t.colors}`;
  if (v.tilesKey === key) return;
  if (v.tiles) v.scene.remove(v.tiles);
  const g = new THREE.Group();
  const rows = Math.ceil(t.road.length / t.cols);
  const ground = new THREE.Mesh(new THREE.PlaneGeometry(t.cols, rows), new THREE.MeshLambertMaterial({ color: 0x1d1f2b }));
  ground.rotation.x = -Math.PI / 2;
  ground.position.set(t.cols / 2, -0.01, rows / 2);
  g.add(ground);
  const pos = [], col = [];
  const colors = t.colors.map((c) => new THREE.Color(c));
  const e = 0.03;
  for (let i = 0; i < t.road.length; i++) {
    const x = i % t.cols, z = Math.floor(i / t.cols), c = colors[t.road[i] === "1" ? 1 : 0];
    const q = [[x + e, z + e], [x + e, z + 1 - e], [x + 1 - e, z + 1 - e], [x + e, z + e], [x + 1 - e, z + 1 - e], [x + 1 - e, z + e]];
    for (const [a, b] of q) { pos.push(a, 0, b); col.push(c.r, c.g, c.b); }
  }
  const geo = new THREE.BufferGeometry();
  geo.setAttribute("position", new THREE.Float32BufferAttribute(pos, 3));
  geo.setAttribute("color", new THREE.Float32BufferAttribute(col, 3));
  geo.computeVertexNormals();
  g.add(new THREE.Mesh(geo, new THREE.MeshLambertMaterial({ vertexColors: true, side: THREE.DoubleSide })));
  v.scene.add(g);
  v.tiles = g;
  v.tilesKey = key;
}

// The pieces: each model's instances pooled and reused; each placed,
// turned and posed at its clip's time, as the scene says.
function pieces(v, list, base) {
  const used = new Map();
  let missing = 0;
  for (const p of list) {
    const url = base + p.m + ".glb";
    const gltf = model(url);
    if (!gltf) { missing++; continue; }
    const pool = v.pools.get(url) || [];
    v.pools.set(url, pool);
    const n = used.get(url) || 0;
    used.set(url, n + 1);
    if (!pool[n]) {
      const obj = gltf.scene.clone(true);
      v.scene.add(obj);
      pool[n] = { obj, mixer: new THREE.AnimationMixer(obj), clip: "", action: null, gltf };
    }
    const it = pool[n];
    it.obj.visible = true;
    it.obj.position.set(p.x, 0, p.z);
    it.obj.rotation.y = Math.atan2(p.dir[0], p.dir[1]);
    if (it.clip !== p.clip) {
      it.mixer.stopAllAction();
      const clip = THREE.AnimationClip.findByName(it.gltf.animations, p.clip);
      it.action = clip ? it.mixer.clipAction(clip) : null;
      if (it.action) it.action.play();
      it.clip = p.clip;
    }
    if (it.action) {
      const d = it.action.getClip().duration || 1;
      it.action.time = ((p.t % d) + d) % d;
      it.mixer.update(0);
    }
  }
  for (const [url, pool] of v.pools) {
    const n = used.get(url) || 0;
    for (let i = n; i < pool.length; i++) pool[i].obj.visible = false;
  }
  return missing;
}

function shots(v, list) {
  while (v.shots.length < list.length) {
    const m = new THREE.Mesh(new THREE.SphereGeometry(0.07, 10, 8), new THREE.MeshBasicMaterial({ color: 0xfff3b0 }));
    v.scene.add(m);
    v.shots.push(m);
  }
  v.shots.forEach((m, i) => {
    m.visible = i < list.length;
    if (m.visible) m.position.set(list[i][0], 0.55, list[i][1]);
  });
}

window.xetalScene = {
  update(canvas, text) {
    try {
      const s = JSON.parse(text);
      const v = viewOf(canvas);
      const w = canvas.clientWidth, h = canvas.clientHeight;
      if (!w || !h) return false;
      v.renderer.setSize(w, h, false);
      v.renderer.setClearColor(s.background || "#000000");
      // The camera: orthographic, looking at the target from dir, k
      // picture units to a world unit, framing the picture's view.
      const [vw, vh] = s.view, k = s.camera.scale, [tx, ty, tz] = s.camera.target, [dx, dy, dz] = s.camera.dir;
      Object.assign(v.camera, { left: -vw / 2 / k, right: vw / 2 / k, top: vh / 2 / k, bottom: -vh / 2 / k });
      v.camera.position.set(tx + 100 * dx, ty + 100 * dy, tz + 100 * dz);
      v.camera.up.set(0, 1, 0);
      v.camera.lookAt(tx, ty, tz);
      v.camera.updateProjectionMatrix();
      if (s.tiles) tiles(v, s.tiles);
      if (v.tiles) v.tiles.visible = !!s.tiles;
      const missing = pieces(v, s.pieces || [], s.base || "");
      shots(v, s.shots || []);
      v.renderer.render(v.scene, v.camera);
      return missing === 0;
    } catch (e) {
      console.warn("xetalScene:", e);
      return false;
    }
  },
};
