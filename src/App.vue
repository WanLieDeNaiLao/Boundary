<script setup lang="ts">
import { ref, computed } from "vue";

// 1. 定义数据结构（这就是我们未来真正的数据格式）
interface Anime {
  id: string;
  title: string;
  episodes: number;
  cover: string;
  isLocalExist: boolean; // 本地是否有文件（控制按钮是否灰显）
  hasUpdate: boolean;    // 云端是否有更新
}

// 2. 模拟前端数据（未来这里的数据是从 Rust 读出来的）
const catalog = ref<Anime[]>([
  {
    id: "1",
    title: "葬送的芙莉莲",
    episodes: 28,
    cover: "https://lain.bgm.tv/pic/cover/l/c2/6d/400602_sG8sS.jpg",
    isLocalExist: true,
    hasUpdate: false,
  },
  {
    id: "2",
    title: "进击的巨人 最终季",
    episodes: 16,
    cover: "https://lain.bgm.tv/pic/cover/l/3f/61/305918_B7Jb2.jpg",
    isLocalExist: false, // 模拟：本地没有这个文件
    hasUpdate: true,
  },
  {
    id: "3",
    title: "孤独摇滚！",
    episodes: 12,
    cover: "https://lain.bgm.tv/pic/cover/l/43/0c/353931_0B0B7.jpg",
    isLocalExist: true,
    hasUpdate: false,
  },
]);

// 3. 页面切换逻辑
const currentTab = ref<"library" | "update">("library");

// 模拟的提示弹窗
const toastMessage = ref("");
const showToast = (msg: string) => {
  toastMessage.value = msg;
  setTimeout(() => {
    toastMessage.value = "";
  }, 3000);
};

// 4. 核心逻辑：点击播放
const handlePlay = (anime: Anime) => {
  if (!anime.isLocalExist) return; // 灰显的不能点
  // 这里未来会调用 Rust 的 invoke('player_play')
  showToast(`🎬 正在调用系统播放器播放：${anime.title}`);
};

// 5. 模拟数据更新检查
const checkUpdates = () => {
  showToast("🔄 正在检查云端更新...（模拟）");
  setTimeout(() => {
    currentTab.value = "update";
  }, 1000);
};

// 统计有多少条更新
const updateCount = computed(() => catalog.value.filter(a => a.hasUpdate).length);
</script>

<template>
  <div class="app-container">
    <!-- 左侧导航 -->
    <aside class="sidebar">
      <h2>Boundary 归档</h2>
      <nav>
        <div 
          class="nav-item" 
          :class="{ active: currentTab === 'library' }"
          @click="currentTab = 'library'"
        >
          📁 本地归档库
        </div>
        <div 
          class="nav-item" 
          :class="{ active: currentTab === 'update' }"
          @click="currentTab = 'update'"
        >
          🔄 数据更新
          <span v-if="updateCount > 0" class="badge">{{ updateCount }}</span>
        </div>
      </nav>
      <div class="sidebar-footer">
        <button class="btn-check" @click="checkUpdates">检查更新</button>
      </div>
    </aside>

    <!-- 右侧主内容区 -->
    <main class="content">
      <!-- ================= 视图 1：本地归档库 ================= -->
      <div v-if="currentTab === 'library'" class="view-animation">
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
              
              <!-- 核心：播放按钮的灰显逻辑 -->
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

      <!-- ================= 视图 2：数据更新中心 ================= -->
      <div v-if="currentTab === 'update'" class="view-animation">
        <h1>数据更新中心</h1>
        <p class="desc">对比本地与云端的数据差异。</p>
        
        <div class="update-list">
          <div class="update-item" v-for="anime in catalog.filter(a => a.hasUpdate)" :key="anime.id">
            <div class="diff-title">
              <h3>{{ anime.title }}</h3>
              <span class="tag updated">内容有变化</span>
            </div>
            <div class="diff-content">
              <p><strong>标题：</strong> <span class="old">进击的巨人 最终季</span> ➡️ <span class="new">进击的巨人 最终季 Part 2</span></p>
              <p><strong>集数：</strong> <span class="old">16</span> ➡️ <span class="new">28</span></p>
            </div>
            <div class="diff-actions">
              <button class="btn-apply" @click="showToast('✅ 更新已应用')">应用更新</button>
              <button class="btn-ignore" @click="showToast('已忽略此版本')">忽略此版本</button>
            </div>
          </div>
          <div v-if="updateCount === 0" class="empty-state">
            所有数据均已是最新。
          </div>
        </div>
      </div>
    </main>

    <!-- 全局 Toast 提示 -->
    <div v-if="toastMessage" class="toast">{{ toastMessage }}</div>
  </div>
</template>

<style>
/* 重置基础样式 */
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; background: #f3f4f6; color: #333; }

.app-container { display: flex; height: 100vh; overflow: hidden; }

/* 侧边栏样式 */
.sidebar { width: 240px; background: #1e1e24; color: #fff; display: flex; flex-direction: column; }
.sidebar h2 { padding: 20px; font-size: 18px; border-bottom: 1px solid #333; }
.nav-item { padding: 15px 20px; cursor: pointer; display: flex; align-items: center; justify-content: space-between; transition: background 0.2s; }
.nav-item:hover { background: #2a2a35; }
.nav-item.active { background: #3b82f6; font-weight: bold; }
.badge { background: #ef4444; color: white; font-size: 12px; padding: 2px 8px; border-radius: 10px; }
.sidebar-footer { margin-top: auto; padding: 20px; }
.btn-check { width: 100%; padding: 10px; background: transparent; border: 1px solid #555; color: #ccc; border-radius: 6px; cursor: pointer; transition: 0.2s; }
.btn-check:hover { background: #333; color: white; }

/* 主内容区 */
.content { flex: 1; padding: 40px; overflow-y: auto; }
h1 { margin-bottom: 20px; font-size: 24px; }

/* 视图动画 */
.view-animation { animation: fadeIn 0.3s ease-out; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }

/* 归档库网格 */
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 24px; }
.anime-card { background: white; border-radius: 12px; overflow: hidden; box-shadow: 0 4px 6px rgba(0,0,0,0.05); transition: transform 0.2s; }
.anime-card:hover { transform: translateY(-5px); }
.cover-wrapper { position: relative; height: 300px; }
.cover-wrapper img { width: 100%; height: 100%; object-fit: cover; }
.update-badge { position: absolute; top: 10px; right: 10px; background: #ef4444; color: white; padding: 4px 10px; border-radius: 12px; font-size: 12px; font-weight: bold; }
.info { padding: 16px; }
.info h3 { font-size: 16px; margin-bottom: 8px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.info p { color: #666; font-size: 14px; margin-bottom: 16px; }

/* 播放按钮核心逻辑 */
.btn-play { width: 100%; padding: 10px; border: none; border-radius: 6px; background: #3b82f6; color: white; font-weight: bold; cursor: pointer; transition: 0.2s; }
.btn-play:hover:not(.disabled) { background: #2563eb; }
.btn-play.disabled { background: #e5e7eb; color: #9ca3af; cursor: not-allowed; }

/* 更新中心 */
.desc { color: #666; margin-bottom: 24px; }
.update-list { display: flex; flex-direction: column; gap: 16px; }
.update-item { background: white; padding: 20px; border-radius: 12px; box-shadow: 0 2px 4px rgba(0,0,0,0.05); }
.diff-title { display: flex; align-items: center; gap: 12px; margin-bottom: 16px; }
.tag.updated { background: #dbeafe; color: #1e40af; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-weight: bold; }
.diff-content { background: #f9fafb; padding: 16px; border-radius: 8px; margin-bottom: 16px; font-size: 14px; line-height: 1.8; }
.old { color: #ef4444; text-decoration: line-through; }
.new { color: #10b981; font-weight: bold; }
.diff-actions { display: flex; gap: 12px; }
.btn-apply { padding: 8px 16px; background: #10b981; color: white; border: none; border-radius: 6px; cursor: pointer; }
.btn-ignore { padding: 8px 16px; background: #f3f4f6; color: #4b5563; border: none; border-radius: 6px; cursor: pointer; }
.empty-state { text-align: center; color: #9ca3af; padding: 40px; background: white; border-radius: 12px; }

/* 全局提示 */
.toast { position: fixed; bottom: 30px; left: 50%; transform: translateX(-50%); background: rgba(0,0,0,0.8); color: white; padding: 12px 24px; border-radius: 30px; animation: fadeIn 0.3s; z-index: 999; }
</style>
