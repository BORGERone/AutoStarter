import { appWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

interface Program {
  id: string;
  name: string;
  path: string;
  enabled: boolean;
  run_minimized: boolean;
  run_hidden_tray: boolean;
  auto_close: boolean;
}

interface AppSettings {
  language: string;
  launch_minimized: boolean;
  app_autostart: boolean;
  close_after_execution: boolean;
}

const translations: Record<string, Record<string, string>> = {
  en: {
    app_title: "Auto",
    app_title_hl: "Start",
    app_autostart: "App autostart",
    add: "Add",
    empty_title: "No programs added",
    empty_desc: 'Click "+ Add" to add applications',
    settings_title: "Settings",
    lang_label: "Language",
    launch_minimized: "Launch to tray",
    settings_info: '"Close after execution" launches a 30-second version at startup.',
    close_after_execution: "Close after execution",
    minimized: "Minimized",
    hidden: "Hidden",
    close: "Close",
    off: "Off",
    on: "On",
    programs: "programs",
    remove: "Remove",
  },
  zh: {
    app_title: "自动",
    app_title_hl: "启动",
    app_autostart: "应用自启",
    add: "添加",
    empty_title: "尚未添加程序",
    empty_desc: '点击"+ 添加"添加应用程序',
    settings_title: "设置",
    lang_label: "语言",
    launch_minimized: "启动到托盘",
    settings_info: '执行后关闭" 在启动时启动 30 秒版本。',
    close_after_execution: "执行后关闭",
    minimized: "最小化",
    hidden: "隐藏",
    close: "关闭",
    off: "关",
    on: "开",
    programs: "程序",
    remove: "删除",
  },
  es: {
    app_title: "Auto",
    app_title_hl: "Inicio",
    app_autostart: "Autoinicio",
    add: "Añadir",
    empty_title: "Sin programas",
    empty_desc: 'Haga clic en "+ Añadir" para agregar',
    settings_title: "Ajustes",
    lang_label: "Idioma",
    launch_minimized: "Iniciar en bandeja",
    settings_info: '"Cerrar tras ejecución" lanza versión de 30 segundos.',
    close_after_execution: "Cerrar tras ejecución",
    minimized: "Minimizado",
    hidden: "Oculto",
    close: "Cerrar",
    off: "Apagado",
    on: "Encendido",
    programs: "programas",
    remove: "Eliminar",
  },
  ar: {
    app_title: "بدء",
    app_title_hl: "التلقائي",
    app_autostart: "تشغيل تلقائي",
    add: "إضافة",
    empty_title: "لم تضف برامج بعد",
    empty_desc: 'انقر على "+ إضافة" لإضافة تطبيقات',
    settings_title: "الإعدادات",
    lang_label: "اللغة",
    launch_minimized: "بدء في العلبة",
    settings_info: '"إغلاق بعد التنفيذ" يشغل نسخة 30 ثانية عند بدء التشغيل.',
    close_after_execution: "إغلاق بعد التنفيذ",
    minimized: "مصغر",
    hidden: "مخفي",
    close: "إغلاق",
    off: "إيقاف",
    on: "تشغيل",
    programs: "برامج",
    remove: "حذف",
  },
  pt: {
    app_title: "Auto",
    app_title_hl: "Iniciar",
    app_autostart: "Autoiniciar",
    add: "Adicionar",
    empty_title: "Nenhum programa",
    empty_desc: 'Clique em "+ Adicionar" para adicionar',
    settings_title: "Configurações",
    lang_label: "Idioma",
    launch_minimized: "Iniciar na bandeja",
    settings_info: '"Fechar após execução" inicia versão de 30 segundos.',
    close_after_execution: "Fechar após execução",
    minimized: "Minimizado",
    hidden: "Oculto",
    close: "Fechar",
    off: "Desligado",
    on: "Ligado",
    programs: "programas",
    remove: "Remover",
  },
  ru: {
    app_title: "Авто",
    app_title_hl: "Запуск",
    app_autostart: "Автозапуск",
    add: "Добавить",
    empty_title: "Программы не добавлены",
    empty_desc: 'Нажмите "+ Добавить" чтобы добавить приложения',
    settings_title: "Настройки",
    lang_label: "Язык",
    launch_minimized: "Запускать в трей",
    settings_info: '"Закрывать после отработки" запускает 30-секундную версию при старте.',
    close_after_execution: "Закрывать после отработки",
    minimized: "Свёрнуто",
    hidden: "Скрыто",
    close: "Закрыть",
    off: "Выкл",
    on: "Вкл",
    programs: "программ",
    remove: "Удалить",
  },
  fr: {
    app_title: "Auto",
    app_title_hl: "Démarrage",
    app_autostart: "Auto-démarrage",
    add: "Ajouter",
    empty_title: "Aucun programme",
    empty_desc: 'Cliquez sur "+ Ajouter" pour ajouter',
    settings_title: "Paramètres",
    lang_label: "Langue",
    launch_minimized: "Lancer dans la barre",
    settings_info: '"Fermer après exécution" lance version 30s au démarrage.',
    close_after_execution: "Fermer après exécution",
    minimized: "Minimisé",
    hidden: "Caché",
    close: "Fermer",
    off: "Arrêt",
    on: "Marche",
    programs: "programmes",
    remove: "Supprimer",
  },
  de: {
    app_title: "Auto",
    app_title_hl: "Start",
    app_autostart: "Autostart",
    add: "Hinzufügen",
    empty_title: "Keine Programme",
    empty_desc: 'Klicken Sie "+ Hinzufügen"',
    settings_title: "Einstellungen",
    lang_label: "Sprache",
    launch_minimized: "In Ablage starten",
    settings_info: '"Schließen nach Ausführung" startet 30-Sekunden-Version.',
    close_after_execution: "Schließen nach Ausführung",
    minimized: "Minimiert",
    hidden: "Versteckt",
    close: "Schließen",
    off: "Aus",
    on: "An",
    programs: "Programme",
    remove: "Entfernen",
  },
  ja: {
    app_title: "自動",
    app_title_hl: "起動",
    app_autostart: "自動起動",
    add: "追加",
    empty_title: "プログラムがありません",
    empty_desc: '"＋追加"をクリックしてアプリを追加',
    settings_title: "設定",
    lang_label: "言語",
    launch_minimized: "トレイに起動",
    settings_info: '"実行後に閉じる" は30秒バージョンを起動します。',
    close_after_execution: "実行後に閉じる",
    minimized: "最小化",
    hidden: "非表示",
    close: "閉じる",
    off: "切",
    on: "入",
    programs: "プログラム",
    remove: "削除",
  },
  hi: {
    app_title: "ऑटो",
    app_title_hl: "स्टार्ट",
    app_autostart: "ऑटोस्टार्ट",
    add: "जोड़ें",
    empty_title: "कोई प्रोग्राम नहीं",
    empty_desc: '"＋ जोड़ें" पर क्लिक करें',
    settings_title: "सेटिंग्स",
    lang_label: "भाषा",
    launch_minimized: "ट्रे में लॉन्च करें",
    settings_info: '"निष्पादन के बाद बंद करें" 30 सेकंड वाला वर्शन लॉन्च करता है।',
    close_after_execution: "निष्पादन के बाद बंद करें",
    minimized: "न्यूनतम",
    hidden: "छिपा हुआ",
    close: "बंद करें",
    off: "बंद",
    on: "चालू",
    programs: "प्रोग्राम",
    remove: "हटाएँ",
  },
};

let currentLang = "en";

function t(key: string): string {
  return translations[currentLang]?.[key] ?? translations["en"]?.[key] ?? key;
}

function applyLanguage(lang: string): void {
  currentLang = lang;
  document.querySelectorAll("[data-i18n]").forEach((el) => {
    const key = el.getAttribute("data-i18n")!;
    el.textContent = t(key);
  });
  const addBtn = document.getElementById("btn-add-program");
  if (addBtn) {
    const span = addBtn.querySelector("[data-i18n]");
    if (span) {
      const key = span.getAttribute("data-i18n")!;
      span.textContent = t(key);
    }
  }
  // Re-render programs to update labels
  renderPrograms(currentPrograms);
}

let currentPrograms: Program[] = [];

// Window controls
document.getElementById("btn-minimize")?.addEventListener("click", () => {
  appWindow.minimize();
});

document.getElementById("btn-maximize")?.addEventListener("click", async () => {
  const isMaximized = await appWindow.isMaximized();
  if (isMaximized) {
    appWindow.unmaximize();
  } else {
    appWindow.maximize();
  }
});

document.getElementById("btn-close")?.addEventListener("click", async () => {
  try {
    await invoke("hide_to_tray");
  } catch (e) {
    console.error("hide_to_tray failed:", e);
  }
});

document.addEventListener("contextmenu", (e) => {
  e.preventDefault();
});

// Settings modal
const settingsModal = document.getElementById("settings-modal")!;
document.getElementById("btn-settings")?.addEventListener("click", () => {
  settingsModal.classList.add("active");
});
document.getElementById("btn-settings-close")?.addEventListener("click", () => {
  settingsModal.classList.remove("active");
});
settingsModal.addEventListener("click", (e) => {
  if (e.target === settingsModal) {
    settingsModal.classList.remove("active");
  }
});

// Add program
document.getElementById("btn-add-program")?.addEventListener("click", async () => {
  const selected = await open({
    multiple: false,
    filters: [{
      name: "Programs",
      extensions: ["exe", "lnk", "bat", "cmd"]
    }]
  });

  if (selected) {
    try {
      const programs: Program[] = await invoke("add_program", { path: selected as string });
      currentPrograms = programs;
      renderPrograms(programs);
    } catch (e) {
      console.error("Failed to add program:", e);
    }
  }
});

// Settings toggles
const settingMinimized = document.getElementById("setting-minimized") as HTMLInputElement;
const settingLang = document.getElementById("setting-lang") as HTMLSelectElement;
const settingAutostart = document.getElementById("setting-autostart") as HTMLInputElement;
const settingCloseAfter = document.getElementById("setting-close-after") as HTMLInputElement;
const settingCloseAfterRow = document.getElementById("setting-close-after-row") as HTMLElement;

let settingsLoaded = false;

function updateCloseAfterDisabled(): void {
  const disabled = !settingAutostart.checked;
  settingCloseAfter.disabled = disabled;
  settingCloseAfterRow.style.opacity = disabled ? "0.5" : "1";
}

async function loadSettings(): Promise<void> {
  try {
    const settings: AppSettings = await invoke("get_settings");
    settingMinimized.checked = settings.launch_minimized;
    settingLang.value = settings.language;
    settingAutostart.checked = settings.app_autostart;
    settingCloseAfter.checked = settings.close_after_execution;
    settingsLoaded = true;
    updateCloseAfterDisabled();
    applyLanguage(settings.language);
  } catch (e) {
    console.error("Failed to load settings:", e);
  }
}

async function saveCurrentSettings(): Promise<void> {
  if (!settingsLoaded) return;
  try {
    await invoke("save_settings", {
      settings: {
        language: settingLang.value,
        launch_minimized: settingMinimized.checked,
        app_autostart: settingAutostart.checked,
        close_after_execution: settingCloseAfter.checked,
      },
    });
  } catch (e) {
    console.error("Failed to save settings:", e);
  }
}

settingLang.addEventListener("change", () => {
  applyLanguage(settingLang.value);
  saveCurrentSettings();
});

settingMinimized.addEventListener("change", saveCurrentSettings);

settingAutostart.addEventListener("change", () => {
  updateCloseAfterDisabled();
  if (!settingAutostart.checked) {
    settingCloseAfter.checked = false;
  }
  saveCurrentSettings();
});

settingCloseAfter.addEventListener("change", saveCurrentSettings);

async function loadPrograms(): Promise<void> {
  try {
    const programs: Program[] = await invoke("get_programs");
    currentPrograms = programs;
    renderPrograms(programs);
  } catch (e) {
    console.error("Failed to load programs:", e);
  }
}

function renderPrograms(programs: Program[]): void {
  const list = document.getElementById("programs-list")!;
  const emptyState = document.getElementById("empty-state")!;
  const footerCount = document.getElementById("footer-count")!;

  footerCount.textContent = `${programs.length} ${t("programs")}`;

  if (programs.length === 0) {
    const existingCards = Array.from(list.querySelectorAll(".program-card:not(.empty-state)"));
    if (existingCards.length > 0) {
      existingCards.forEach(c => c.classList.add("removing"));
      setTimeout(() => {
        existingCards.forEach(c => c.remove());
        emptyState.style.display = "flex";
        emptyState.style.animation = "none";
        requestAnimationFrame(() => { emptyState.style.animation = ""; });
      }, 300);
    } else {
      emptyState.style.display = "flex";
    }
    return;
  }

  emptyState.style.display = "none";

  const existingCards = Array.from(list.querySelectorAll(".program-card:not(.empty-state)"));
  if (existingCards.length === 0) {
    for (const program of programs) {
      const card = createProgramCard(program);
      list.appendChild(card);
    }
    return;
  }

  const existingIds = new Set(existingCards.map(c => (c as HTMLElement).dataset.id!));
  const newIds = new Set(programs.map(p => p.id));

  const toRemove = existingCards.filter(c => !newIds.has((c as HTMLElement).dataset.id!));
  const toAdd = programs.filter(p => !existingIds.has(p.id));

  if (toRemove.length > 0) {
    toRemove.forEach(c => c.classList.add("removing"));
    setTimeout(() => {
      toRemove.forEach(c => c.remove());
      for (const program of toAdd) {
        const card = createProgramCard(program);
        list.appendChild(card);
      }
    }, 300);
  } else {
    for (const program of toAdd) {
      const card = createProgramCard(program);
      list.appendChild(card);
    }
  }
}

function createProgramCard(program: Program): HTMLElement {
  const card = document.createElement("div");
  card.className = "program-card";
  card.dataset.id = program.id;

  card.innerHTML = `
    <div class="program-icon">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="18" height="18" rx="2"/>
        <path d="M9 3v18"/>
        <path d="M3 9h18"/>
      </svg>
    </div>
    <div class="program-info">
      <div class="program-name">${escapeHtml(program.name)}</div>
      <div class="program-path">${escapeHtml(program.path)}</div>
    </div>
    <div class="program-options">
      <label class="checkbox-wrapper">
        <input type="checkbox" class="chk-minimized" ${program.run_minimized ? "checked" : ""}>
        <span class="checkbox-custom"></span>
        <span class="checkbox-label" data-i18n-label="minimized">${t("minimized")}</span>
      </label>
      <label class="checkbox-wrapper">
        <input type="checkbox" class="chk-hidden" ${program.run_hidden_tray ? "checked" : ""}>
        <span class="checkbox-custom"></span>
        <span class="checkbox-label" data-i18n-label="hidden">${t("hidden")}</span>
      </label>
      <label class="checkbox-wrapper">
        <input type="checkbox" class="chk-close" ${program.auto_close ? "checked" : ""}>
        <span class="checkbox-custom"></span>
        <span class="checkbox-label" data-i18n-label="close">${t("close")}</span>
      </label>
      <label class="toggle-wrapper">
        <span class="toggle-label">${t("off")}</span>
        <span class="toggle">
          <input type="checkbox" class="tgl-enabled" ${program.enabled ? "checked" : ""}>
          <span class="toggle-slider"></span>
        </span>
        <span class="toggle-label">${t("on")}</span>
      </label>
      <button class="program-delete" title="${t("remove")}">
        <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M2 4h12M5 4V2.5a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1V4M12 4v9.5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V4"/>
          <path d="M6 7v5M10 7v5"/>
        </svg>
      </button>
    </div>
  `;

  // Re-apply i18n for labels created dynamically
  card.querySelectorAll("[data-i18n-label]").forEach((el) => {
    const key = el.getAttribute("data-i18n-label")!;
    el.textContent = t(key);
  });

  const toggle = card.querySelector(".tgl-enabled") as HTMLInputElement;
  toggle.addEventListener("change", async () => {
    try {
      const programs: Program[] = await invoke("toggle_program", {
        id: program.id,
        enabled: toggle.checked,
      });
      currentPrograms = programs;
      renderPrograms(programs);
    } catch (e) {
      toggle.checked = !toggle.checked;
    }
  });

  const chkMinimized = card.querySelector(".chk-minimized") as HTMLInputElement;
  const chkHidden = card.querySelector(".chk-hidden") as HTMLInputElement;
  const chkClose = card.querySelector(".chk-close") as HTMLInputElement;

  const updateOptions = async () => {
    const minimized = chkMinimized.checked;
    const hidden = chkHidden.checked;
    try {
      const programs: Program[] = await invoke("update_program_options", {
        id: program.id,
        runMinimized: hidden ? false : minimized,
        runHiddenTray: hidden,
        autoClose: chkClose.checked,
      });
      currentPrograms = programs;
      renderPrograms(programs);
    } catch (e) {
      console.error("Failed to update options:", e);
    }
  };

  chkMinimized.addEventListener("change", async () => {
    if (chkMinimized.checked) {
      chkHidden.checked = false;
    }
    await updateOptions();
  });

  chkHidden.addEventListener("change", async () => {
    if (chkHidden.checked) {
      chkMinimized.checked = false;
    }
    await updateOptions();
  });

  chkClose.addEventListener("change", async () => {
    await updateOptions();
  });

  const deleteBtn = card.querySelector(".program-delete")!;
  deleteBtn.addEventListener("click", async () => {
    try {
      const programs: Program[] = await invoke("remove_program", { id: program.id });
      currentPrograms = programs;
      renderPrograms(programs);
    } catch (e) {
      console.error("Failed to remove program:", e);
    }
  });

  return card;
}

function escapeHtml(text: string): string {
  const div = document.createElement("div");
  div.textContent = text;
  return div.innerHTML;
}

loadPrograms();
loadSettings();
