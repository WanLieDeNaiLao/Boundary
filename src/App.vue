<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// ==========================================
// 1. 数据结构与配置 (参考 Kazumi 的 MenuRoute)
// ==========================================
interface Anime {
  id: string;
  title: string;
  episodes: number;
  cover: string;
  filePath: string;
  isLocalExist: boolean;
  hasUpdate: boolean;
}

interface MenuRouteItem {
  path: string;
  label: string;
  icon: string;
}

// 底部导航菜单配置
const menuRoutes: MenuRouteItem[] = [
  { path: "home", label: "主页", icon: "🏠" },
  { path: "timeline", label: "时间表", icon: "📅" },
  { path: "favorite", label: "爱好", icon: "⭐" },
  { path: "settings", label: "设置", icon: "⚙️" },
];

// ==========================================
// 2. 模拟初始化与数据加载 (参考 Kazumi 的 InitPage)
// ==========================================
const isInitializing = ref(true); // 增加一个启动加载状态
const currentTab = ref<string>("home"); // 当前选中的路由 path

const catalog = ref<Anime[]>([
  { id: "1", title: "葬送的芙莉莲", episodes: 28, cover: "https://lain.bgm.tv/pic/cover/l/c2/6d/400602_sG8sS.jpg", filePath: "D:\\TestVideos\\frieren.mp4", isLocalExist: false, hasUpdate: false },
  { id: "2", title: "进击的巨人 最终季", episodes: 16, cover: "https://lain.bgm.tv/pic/cover/l/3f/61/305918_B7Jb2.jpg", filePath: "D:\\TestVideos\\jujutsu.mp4", isLocalExist: false, hasUpdate: true },
  { id: "3", title: "孤独摇滚！", episodes: 12, cover: "https://lain.bgm.tv/pic/cover/l/43/0c/353931_0B0B7.jpg", filePath: "D:\\TestVideos\\bocchi.mp4", isLocalExist: false, hasUpdate: false },
]);

onMounted(async () => {
  // 模拟启动检查耗时
  await new Promise(resolve => setTimeout(resolve, 800)); 
  
  // 向 Rust 检查本地文件是否存在
  for (let anime of catalog.value) {
    try {
      const exists = await invoke<boolean>("check_file_exists", { path: anime.filePath });
      anime.isLocalExist = exists;
    } catch (e) {
      console.error(e);
    }
  }
  
  // 检查完毕，关闭加载页
  isInitializing.value = false;
});

// ==========================================
// 3. 交互逻辑
// ==========================================
// 搜索/返回 按钮状态
const isSearchMode = ref(false);

const handleNavClick = (path: string) => {
  currentTab.value = path;
  isSearchMode.value = false;
};

const toggleSearch = () => {
  isSearchMode.value = !isSearchMode.value;
  if (!isSearchMode.value) {
    currentTab.value = "home"; // 退出搜索回到主页
  }
};

// 弹窗逻辑
const toastMessage = ref("");
const showToast = (msg: string) => {
  toastMessage.value = msg;
  setTimeout(() => { toastMessage.value = ""; }, 3000);
};

const handlePlay = (anime: Anime) => {
  if (!anime.isLocalExist) return;
  showToast(`🎬 正在调用系统播放器播放：${anime.title}`);
};
</script>

<template>
  <!-- 🌟 模拟 InitPage：启动加载遮罩 -->
  <div v-if="isInitializing" class="loading-screen">
    <div class="spinner"></div>
    <p>Boundary 正在初始化...</p>
  </div>

  <!-- 🌟 主应用容器 -->
  <div v-else class="app-container">
    
    <!-- 左侧导航 -->
    <aside class="sidebar">
      <!-- 顶部：搜索 / 返回 -->
      <div class="nav-section top-section">
        <div 
          class="nav-item search-btn" 
          :class="{ active: isSearchMode }"
          @click="toggleSearch"
        >
          <span class="icon">{{ isSearchMode ? '⬅️' : '🔍' }}</span>
          <span class="label">{{ isSearchMode ? '返回' : '搜索' }}</span>
        </div>
      </div>

      <!-- 中间留白 -->
      <div class="spacer"></div>

      <!-- 底部导航（搜索模式下隐藏） -->
      <div class="nav-section bottom-section" v-if="!isSearchMode">
        <div 
          v-for="route in menuRoutes" 
          :key="route.path"
          class="nav-item" 
          :class="{ active: currentTab === route.path }"
          @click="handleNavClick(route.path)"
        >
          <span class="icon">{{ route.icon }}</span>
          <span class="label">{{ route.label }}</span>
        </div>
      </div>
    </aside>

    <!-- 右侧主内容区 -->
    <main class="content-wrapper">
      <div class="content-box">
        
        <!-- 搜索页面 -->
        <div v-if="isSearchMode" class="view-animation">
          <h1>🔍 全局搜索</h1>
          <p class="placeholder-text">这里是独立的搜索界面，左侧导航栏已隐藏。</p>
        </div>

        <!-- 主页 -->
        <div v-else-if="currentTab === 'home'" class="view-animation">
          <h1>我的视频库</h1>
          <div class="grid">
            <div class="anime-card" v-for="anime in catalog" :key="anime.id">
              <div class="cover-wrapper">
                <img :src="anime.cover" :alt="anime.title" />
                <span v-if="anime.hasUpdate" class="update-badge">有更新</span>
              </div>
              <div class="info">
                <h3>{{ anime.title }}</h3>
                <p>全 {{ anime.episodes }} 集</p>
                <button 
                  class="btn-play" 
                  :class="{ disabled: !anime.isLocalExist }"
                  :disabled="!anime.isLocalExist"
                  @click="handlePlay(anime)"
                >
                  {{ anime.isLocalExist ? '▶ 播放' : '未找到本地文件' }}
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- 其他占位页面 -->
        <div v-else class="view-animation placeholder-view">
          <h1>{{ menuRoutes.find(r => r.path === currentTab)?.label }}</h1>
          <p class="placeholder-text">该模块正在开发中，敬请期待...</p>
        </div>

      </div>
    </main>

    <div v-if="toastMessage" class="toast">{{ toastMessage }}</div>
  </div>
</template>

<style>
/* ====================================================
   🌟 核心控制台：所有样式、尺寸、颜色都在这里调
==================================================== */
:root {
  --sidebar-width: 80px;       
  --nav-item-size: 54px;       
  --nav-radius: 12px;          
  --content-radius: 24px;      
  --font-size: 12px;           
  
  --bg-app: rgb(18, 20, 18);   
  --bg-sidebar: rgb(29, 33, 27); 
  --bg-content: rgb(16, 20, 15); 
  --bg-active: rgb(63, 80, 60);  
  --bg-search: rgb(35, 80, 36);  
  
  --text-main: #f3f4f6;        
  --text-sub: #9ca3af;         
}

* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: var(--bg-app); color: var(--text-main); overflow: hidden; }

/* 🌟 初始化 Loading 界面 */
.loading-screen {
  display: flex; flex-direction: column; align-items: center; justify-content: center;
  height: 100vh; background: var(--bg-app); color: var(--text-sub);
}
.spinner {
  width: 40px; height: 40px; border: 4px solid rgba(255,255,255,0.1);
  border-top-color: var(--bg-active); border-radius: 50%;
  animation: spin 1s linear infinite; margin-bottom: 20px;
}
@keyframes spin { to { transform: rotate(360deg); } }

/* ================= 主应用容器 ================= */
.app-container { display: flex; height: 100vh; }

/* ================= 侧边栏 ================= */
.sidebar { 
  width: var(--sidebar-width); 
  background: var(--bg-sidebar); 
  display: flex; flex-direction: column; 
  padding: 20px 0; align-items: center; 
}
.nav-section { display: flex; flex-direction: column; gap: 12px; }
.spacer { flex: 1; } 

/* 🌟 导航按钮 */
.nav-item {
  position: relative; width: var(--nav-item-size); height: var(--nav-item-size);
  display: flex; flex-direction: column; align-items: center; justify-content: center;
  border-radius: var(--nav-radius); cursor: pointer; overflow: hidden; 
  user-select: none; color: var(--text-sub);
}
.nav-item::before {
  content: ''; position: absolute; inset: 0; background: var(--bg-active);
  border-radius: var(--nav-radius); transform: scale(0); opacity: 0;
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.3s; z-index: 0;
}
.nav-item.active::before { transform: scale(1); opacity: 1; }
.nav-item:hover { color: var(--text-main); }

.nav-item .icon {
  position: relative; z-index: 1; font-size: 20px;
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}
.nav-item .label {
  position: absolute; bottom: 6px; z-index: 1; font-size: var(--font-size);
  font-weight: 500; opacity: 0; transform: translateY(10px); 
  transition: opacity 0.3s, transform 0.3s; white-space: nowrap;
}
.nav-item.active { color: var(--text-main); }
.nav-item.active .icon { transform: translateY(-8px); }
.nav-item.active .label { opacity: 1; transform: translateY(0); }

.search-btn { background: var(--bg-search); color: var(--text-main); }
.search-btn:hover { background: rgb(45, 100, 46); }
.search-btn.active { background: transparent; } 

/* ================= 右侧内容区 ================= */
.content-wrapper { flex: 1; padding: 16px; padding-left: 0; display: flex; }
.content-box {
  flex: 1; background: var(--bg-content); border-radius: var(--content-radius);
  padding: 40px; overflow-y: auto; box-shadow: 0 10px 30px rgba(0,0,0,0.2);
}

h1 { margin-bottom: 24px; font-size: 24px; font-weight: 600; }
.placeholder-view { display: flex; flex-direction: column; align-items: center; justify-content: center; height: 80%; }
.placeholder-text { color: var(--text-sub); }

.view-animation { animation: fadeIn 0.3s ease-out; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }

.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 24px; }
.anime-card { background: rgba(255,255,255,0.05); border-radius: 12px; overflow: hidden; border: 1px solid rgba(255,255,255,0.1); transition: transform 0.2s; }
.anime-card:hover { transform: translateY(-4px); background: rgba(255,255,255,0.08); }
.cover-wrapper { position: relative; height: 280px; }
.cover-wrapper img { width: 100%; height: 100%; object-fit: cover; }
.update-badge { position: absolute; top: 10px; right: 10px; background: #ef4444; color: white; padding: 4px 10px; border-radius: 12px; font-size: 12px; font-weight: bold; }
.info { padding: 16px; }
.info h3 { font-size: 15px; margin-bottom: 6px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; color: var(--text-main); }
.info p { color: var(--text-sub); font-size: 13px; margin-bottom: 16px; }

.btn-play { width: 100%; padding: 10px; border: none; border-radius: 6px; background: #3b82f6; color: white; font-weight: bold; cursor: pointer; transition: 0.2s; }
.btn-play:hover:not(.disabled) { background: #2563eb; }
.btn-play.disabled { background: rgba(255,255,255,0.1); color: rgba(255,255,255,0.3); cursor: not-allowed; }

.toast { position: fixed; bottom: 30px; left: 50%; transform: translateX(-50%); background: rgba(0,0,0,0.8); color: white; padding: 12px 24px; border-radius: 30px; animation: fadeIn 0.3s; z-index: 999; }
</style>