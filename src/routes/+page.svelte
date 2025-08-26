<script lang="ts">
  import "../app.css"
  import TabContent from './TabContent.svelte';
  import SettingContent from './SettingContent.svelte';
  
  let tabs = [
    { id: 1, title: "首页" },
  ];
  
  let activeTab = 1;
  
  // 为每个标签页维护独立的组件实例
  let tabInstances = new Map();
  
  function closeTab(tabId) {
    tabs = tabs.filter(tab => tab.id !== tabId);
    // 清理标签页组件实例
    tabInstances.delete(tabId);
    if (activeTab === tabId && tabs.length > 0) {
      activeTab = tabs[0].id;
    }
  }
  
  // 切换标签页的mode
  function toggleMode(tabId) {
    const instance = getTabInstance(tabId);
    if (!instance.mode) {
      instance.mode = 'login';
    }
    
    if (instance.mode === 'login') {
      instance.mode = 'roomlist';
    } else {
      instance.mode = 'login';
    }
  }
  
  function addTab() {
    const newId = tabs.length > 0 ? Math.max(...tabs.map(t => t.id)) + 1 : 1;
    tabs = [...tabs, { id: newId, title: `新标签 ${newId}` }];
  }
  
  function addSettingTab() {
    // 检查是否已存在设置标签页
    const existingSettingTab = tabs.find(tab => tab.title.includes("设置"));
    
    if (existingSettingTab) {
      // 如果存在设置标签页，则跳转到该标签页
      activeTab = existingSettingTab.id;
    } else {
      // 如果不存在设置标签页，则创建新的设置标签页
      const newId = tabs.length > 0 ? Math.max(...tabs.map(t => t.id)) + 1 : 1;
      const tabId = newId;
      tabs = [...tabs, { id: tabId, title: `设置 ${tabId}` }];
      // 设置新标签页为激活状态
      activeTab = tabId;
    }
  }
  
  // 获取或创建标签页组件实例
  function getTabInstance(tabId) {
    if (!tabInstances.has(tabId)) {
      // 创建新的组件实例占位符
      tabInstances.set(tabId, {});
    }
    return tabInstances.get(tabId);
  }
  
  // 确定使用哪个组件来渲染标签页内容
  function getComponentForTab(tabId) {
    const tab = tabs.find(t => t.id === tabId);
    // 如果标签页标题包含"设置"，则使用SettingContent组件
    if (tab && tab.title.includes("设置")) {
      return SettingContent;
    }
    // 默认使用TabContent组件
    return TabContent;
  }
</script>

<div class="navbar bg-base-100">
  <div class="navbar-start">
    <div role="tablist" class="tabs tabs-lifted">
      {#each tabs as tab (tab.id)}
        <div class="tab flex items-center {activeTab === tab.id ? 'tab-active' : ''}">
          <button 
            role="tab"
            class="flex-1 text-left"
            on:click={() => activeTab = tab.id}
          >
            {tab.title}
          </button>
          <button 
            class="btn btn-xs btn-circle btn-ghost ml-2" 
            aria-label="关闭标签页 {tab.title}"
            on:click|stopPropagation={() => closeTab(tab.id)}
          >
            ✕
          </button>
        </div>
      {/each}
      <button 
        role="tab"
        class="tab tab-plus"
        aria-label="添加新标签页"
        on:click={addTab}
      >
        <span class="text-xl">+</span>
      </button>
    </div>
  </div>
  <div class="navbar-end">
    <button class="btn btn-ghost btn-sm rounded-btn" aria-label="设置" on:click={addSettingTab}>设置</button>
    <button class="btn btn-ghost btn-sm rounded-btn" aria-label="最小化">最小化</button>
    <button class="btn btn-ghost btn-sm rounded-btn" aria-label="最大化">最大化</button>
    <button class="btn btn-ghost btn-sm rounded-btn" aria-label="关闭">关闭</button>
  </div>
</div>

<main>
  {#each tabs as tab (tab.id)}
    {#if activeTab === tab.id}
      <svelte:component 
        this={getComponentForTab(tab.id)} 
        tabId={tab.id} 
        instance={getTabInstance(tab.id)}
        mode={getTabInstance(tab.id).mode}
      />
    {/if}
  {/each}
</main>

<style>

</style>
