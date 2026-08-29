(() => {
  const tauri = window.__TAURI__ || null;
  const invoke = tauri?.core?.invoke;
  const convertFileSrc = tauri?.core?.convertFileSrc;

  const state = {
    files: [],
    outputFolder: '',
    processing: false,
    results: [],
    settings: loadSettings(),
  };

  const $ = (id) => document.getElementById(id);
  const els = {
    dropzone: $('dropzone'), workspace: $('workspace'), fileList: $('fileList'), fileCount: $('fileCount'),
    chooseBtn: $('chooseBtn'), addMoreBtn: $('addMoreBtn'), clearBtn: $('clearBtn'), processBtn: $('processBtn'),
    format: $('format'), quality: $('quality'), qualityValue: $('qualityValue'), targetToggle: $('targetToggle'),
    targetWrap: $('targetWrap'), targetKb: $('targetKb'), resizeToggle: $('resizeToggle'), resizeWrap: $('resizeWrap'),
    width: $('width'), height: $('height'), keepAspect: $('keepAspect'), outputFolderBtn: $('outputFolderBtn'),
    outputFolderText: $('outputFolderText'), suffix: $('suffix'), progressWrap: $('progressWrap'), progressBar: $('progressBar'),
    progressText: $('progressText'), resultSummary: $('resultSummary'), totalOriginal: $('totalOriginal'),
    totalOutput: $('totalOutput'), totalSaved: $('totalSaved'), openFolderBtn: $('openFolderBtn'), toast: $('toast'),
    historyList: $('historyList'), clearHistoryBtn: $('clearHistoryBtn'), theme: $('theme'), overwrite: $('overwrite'),
    openAfter: $('openAfter'), saveSettingsBtn: $('saveSettingsBtn')
  };

  function formatBytes(bytes) {
    if (!Number.isFinite(bytes) || bytes <= 0) return '0 B';
    const units = ['B','KB','MB','GB'];
    const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
    const value = bytes / Math.pow(1024, i);
    return `${value >= 100 || i === 0 ? value.toFixed(0) : value.toFixed(1)} ${units[i]}`;
  }

  function escapeHtml(value) {
    return String(value).replace(/[&<>'"]/g, (c) => ({'&':'&amp;','<':'&lt;','>':'&gt;',"'":'&#39;','"':'&quot;'}[c]));
  }

  function basename(path) {
    return path.split(/[\\/]/).pop() || path;
  }

  function toast(message, type = 'ok') {
    els.toast.textContent = message;
    els.toast.classList.toggle('error', type === 'error');
    els.toast.classList.add('show');
    clearTimeout(toast.timer);
    toast.timer = setTimeout(() => els.toast.classList.remove('show'), 2600);
  }

  function loadSettings() {
    try {
      return { theme: 'system', overwrite: false, openAfter: false, ...JSON.parse(localStorage.getItem('ic-settings') || '{}') };
    } catch { return { theme: 'system', overwrite: false, openAfter: false }; }
  }

  function applyTheme() {
    const theme = state.settings.theme || 'system';
    if (theme === 'system') {
      document.documentElement.dataset.theme = matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    } else document.documentElement.dataset.theme = theme;
  }

  function saveSettings() {
    state.settings = { theme: els.theme.value, overwrite: els.overwrite.checked, openAfter: els.openAfter.checked };
    localStorage.setItem('ic-settings', JSON.stringify(state.settings));
    applyTheme();
    toast('Settings saved');
  }

  function renderFiles() {
    const hasFiles = state.files.length > 0;
    els.dropzone.classList.toggle('hidden', hasFiles);
    els.workspace.classList.toggle('hidden', !hasFiles);
    els.clearBtn.disabled = !hasFiles || state.processing;
    els.fileCount.textContent = `${state.files.length} image${state.files.length === 1 ? '' : 's'}`;

    const resultsByInput = new Map(state.results.map(r => [r.input_path, r]));
    els.fileList.innerHTML = state.files.map((file, index) => {
      const result = resultsByInput.get(file.path);
      const imgSrc = convertFileSrc ? convertFileSrc(file.path) : '';
      const savings = result && !result.error ? Math.max(0, Math.round((1 - result.output_bytes / result.original_bytes) * 100)) : 0;
      let resultLine = '';
      if (result?.error) resultLine = `<div class="file-result error">${escapeHtml(result.error)}</div>`;
      else if (result) resultLine = `<div class="file-result">${formatBytes(result.original_bytes)} → ${formatBytes(result.output_bytes)} · ${savings}% smaller</div>`;
      return `<div class="file-row">
        ${imgSrc ? `<img class="file-thumb" src="${escapeHtml(imgSrc)}" alt="" />` : `<div class="file-thumb"></div>`}
        <div class="file-meta">
          <div class="file-name" title="${escapeHtml(file.path)}">${escapeHtml(file.name)}</div>
          <div class="file-details"><span>${formatBytes(file.size)}</span><span>${file.width}×${file.height}</span><span>${escapeHtml(file.format.toUpperCase())}</span></div>
          ${resultLine}
        </div>
        <button class="remove-btn" data-remove="${index}" title="Remove">×</button>
      </div>`;
    }).join('');

    els.fileList.querySelectorAll('[data-remove]').forEach(btn => btn.addEventListener('click', () => {
      if (state.processing) return;
      state.files.splice(Number(btn.dataset.remove), 1);
      state.results = [];
      els.resultSummary.classList.add('hidden');
      renderFiles();
    }));
  }

  async function addPaths(paths) {
    if (!invoke) return toast('Desktop runtime is required to read file paths.', 'error');
    const valid = [...new Set(paths)].filter(p => /\.(jpe?g|png|webp)$/i.test(p));
    if (!valid.length) return toast('Choose JPG, PNG or WebP images.', 'error');
    try {
      const infos = await invoke('get_images_info', { paths: valid });
      const existing = new Set(state.files.map(f => f.path));
      state.files.push(...infos.filter(f => !existing.has(f.path)));
      state.results = [];
      els.resultSummary.classList.add('hidden');
      renderFiles();
    } catch (err) { toast(String(err), 'error'); }
  }

  async function chooseImages() {
    if (!invoke) return toast('Run the Tauri desktop app to choose files.', 'error');
    try {
      const paths = await invoke('pick_images');
      if (paths?.length) await addPaths(paths);
    } catch (err) { toast(String(err), 'error'); }
  }

  async function chooseOutputFolder() {
    if (!invoke) return toast('Run the desktop app to choose a folder.', 'error');
    try {
      const path = await invoke('pick_output_folder');
      if (path) {
        state.outputFolder = path;
        els.outputFolderText.textContent = path;
        els.outputFolderText.title = path;
      }
    } catch (err) { toast(String(err), 'error'); }
  }

  function buildOptions() {
    return {
      format: els.format.value,
      quality: Number(els.quality.value),
      target_kb: els.targetToggle.checked ? Math.max(10, Number(els.targetKb.value || 200)) : null,
      resize_enabled: els.resizeToggle.checked,
      width: els.resizeToggle.checked && Number(els.width.value) > 0 ? Number(els.width.value) : null,
      height: els.resizeToggle.checked && Number(els.height.value) > 0 ? Number(els.height.value) : null,
      keep_aspect: els.keepAspect.checked,
      suffix: els.suffix.value.trim() || '-compressed',
      overwrite: !!state.settings.overwrite,
    };
  }

  async function processImages() {
    if (!invoke || state.processing || !state.files.length) return;
    if (!state.outputFolder) {
      await chooseOutputFolder();
      if (!state.outputFolder) return;
    }

    state.processing = true;
    els.processBtn.disabled = true;
    els.clearBtn.disabled = true;
    els.progressWrap.classList.remove('hidden');
    els.progressBar.style.width = '8%';
    els.progressText.textContent = `Processing ${state.files.length} image${state.files.length === 1 ? '' : 's'}…`;
    try {
      const payload = { paths: state.files.map(f => f.path), outputDir: state.outputFolder, options: buildOptions() };
      els.progressBar.style.width = '35%';
      const results = await invoke('process_images', payload);
      state.results = results;
      const ok = results.filter(r => !r.error);
      const failed = results.filter(r => r.error);
      els.progressBar.style.width = '100%';
      els.progressText.textContent = 'Done';
      showSummary(ok);
      renderFiles();
      addToHistory(ok);
      if (failed.length && ok.length) toast(`${ok.length} processed, ${failed.length} failed`, 'error');
      else if (failed.length) toast(`${failed.length} image${failed.length === 1 ? '' : 's'} failed to process`, 'error');
      else toast(`${ok.length} image${ok.length === 1 ? '' : 's'} processed`);
      if (state.settings.openAfter && ok.length) await openFolder();
    } catch (err) {
      toast(String(err), 'error');
      els.progressText.textContent = 'Processing failed';
    } finally {
      state.processing = false;
      els.processBtn.disabled = false;
      els.clearBtn.disabled = false;
      setTimeout(() => els.progressWrap.classList.add('hidden'), 1200);
    }
  }

  function showSummary(results) {
    if (!results.length) { els.resultSummary.classList.add('hidden'); return; }
    const original = results.reduce((sum, r) => sum + r.original_bytes, 0);
    const output = results.reduce((sum, r) => sum + r.output_bytes, 0);
    const saved = original ? Math.round((1 - output / original) * 100) : 0;
    els.totalOriginal.textContent = formatBytes(original);
    els.totalOutput.textContent = formatBytes(output);
    els.totalSaved.textContent = `${Math.max(0, saved)}%`;
    els.resultSummary.classList.remove('hidden');
  }

  async function openFolder() {
    if (!invoke || !state.outputFolder) return;
    try { await invoke('open_folder', { path: state.outputFolder }); }
    catch (err) { toast(String(err), 'error'); }
  }

  function addToHistory(results) {
    const old = readHistory();
    const now = new Date().toISOString();
    const entries = results.map(r => ({ date: now, name: basename(r.output_path), original: r.original_bytes, output: r.output_bytes, path: r.output_path }));
    localStorage.setItem('ic-history', JSON.stringify([...entries, ...old].slice(0, 100)));
    renderHistory();
  }

  function readHistory() {
    try { return JSON.parse(localStorage.getItem('ic-history') || '[]'); } catch { return []; }
  }

  function renderHistory() {
    const items = readHistory();
    if (!items.length) {
      els.historyList.innerHTML = '<div class="empty-state">No processed images yet.</div>';
      return;
    }
    els.historyList.innerHTML = items.map(item => {
      const saving = item.original ? Math.max(0, Math.round((1 - item.output / item.original) * 100)) : 0;
      return `<div class="history-item"><div><strong>${escapeHtml(item.name)}</strong><small>${new Date(item.date).toLocaleString()}</small></div><div class="history-stat"><span>Original</span><b>${formatBytes(item.original)}</b></div><div class="history-stat"><span>Output</span><b>${formatBytes(item.output)}</b></div><div class="history-stat"><span>Saved</span><b>${saving}%</b></div></div>`;
    }).join('');
  }

  function clearAll() {
    if (state.processing) return;
    state.files = []; state.results = [];
    els.resultSummary.classList.add('hidden');
    renderFiles();
  }

  function initNavigation() {
    document.querySelectorAll('.nav-item').forEach(btn => btn.addEventListener('click', () => {
      document.querySelectorAll('.nav-item').forEach(b => b.classList.toggle('active', b === btn));
      document.querySelectorAll('.view').forEach(v => v.classList.remove('active'));
      $(`${btn.dataset.view}View`).classList.add('active');
      if (btn.dataset.view === 'history') renderHistory();
    }));
  }

  async function initDragDrop() {
    try {
      const current = tauri?.webview?.getCurrentWebview?.();
      if (!current?.onDragDropEvent) return;
      await current.onDragDropEvent((event) => {
        const type = event.payload?.type;
        els.dropzone.classList.toggle('dragging', type === 'enter' || type === 'over');
        if (type === 'drop') addPaths(event.payload.paths || []);
        if (type === 'leave' || type === 'drop') els.dropzone.classList.remove('dragging');
      });
    } catch { /* Drag/drop is optional; chooser remains available. */ }
  }

  els.chooseBtn.addEventListener('click', chooseImages);
  els.addMoreBtn.addEventListener('click', chooseImages);
  els.dropzone.addEventListener('click', chooseImages);
  els.dropzone.addEventListener('keydown', e => { if (e.key === 'Enter' || e.key === ' ') chooseImages(); });
  els.clearBtn.addEventListener('click', clearAll);
  els.quality.addEventListener('input', () => els.qualityValue.textContent = els.quality.value);
  els.targetToggle.addEventListener('change', () => els.targetWrap.classList.toggle('hidden', !els.targetToggle.checked));
  els.resizeToggle.addEventListener('change', () => els.resizeWrap.classList.toggle('hidden', !els.resizeToggle.checked));
  els.outputFolderBtn.addEventListener('click', chooseOutputFolder);
  els.processBtn.addEventListener('click', processImages);
  els.openFolderBtn.addEventListener('click', openFolder);
  els.clearHistoryBtn.addEventListener('click', () => { localStorage.removeItem('ic-history'); renderHistory(); toast('History cleared'); });
  els.saveSettingsBtn.addEventListener('click', saveSettings);

  els.theme.value = state.settings.theme;
  els.overwrite.checked = state.settings.overwrite;
  els.openAfter.checked = state.settings.openAfter;
  applyTheme();
  matchMedia('(prefers-color-scheme: dark)').addEventListener?.('change', () => { if (state.settings.theme === 'system') applyTheme(); });
  initNavigation();
  initDragDrop();
  renderFiles();
  renderHistory();

  if (!tauri) toast('UI preview mode. Desktop file processing requires the packaged app.');
})();
