import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

class Updater {
  update = $state<Update | null>(null);
  dismissed = $state(false);
  installing = $state(false);
  downloaded = $state(0);
  total = $state<number | null>(null);
  error = $state<string | null>(null);
  percent = $derived(this.total ? Math.min(100, Math.round((this.downloaded / this.total) * 100)) : null);
  visible = $derived(this.update !== null && !this.dismissed);

  async check() {
    try {
      this.update = await check();
    } catch (err) {
      // Offline, rate-limited or no published release yet: never block the editor.
      console.warn("Update check failed", err);
    }
  }

  async install() {
    if (!this.update || this.installing) return;
    this.installing = true;
    this.error = null;
    this.downloaded = 0;
    this.total = null;
    try {
      await this.update.downloadAndInstall((event) => {
        if (event.event === "Started") this.total = event.data.contentLength ?? null;
        else if (event.event === "Progress") this.downloaded += event.data.chunkLength;
      });
      await relaunch();
    } catch (err) {
      this.error = String(err);
      this.installing = false;
    }
  }
}

export const updater = new Updater();
