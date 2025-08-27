<script>
  // 接收传递的数据
  let { data } = $props();
  
  // 房间列表组件逻辑
</script>

<div class="roomlist-container">
  <h2>房间列表页面</h2>
  <p>这是房间列表页面的内容</p>
  
  <!-- 显示传递的数据 -->
  {#if data}
    <div class="data-display">
      <h3>接收到的数据:</h3>
      <pre>{JSON.stringify(data, null, 2)}</pre>
    </div>
  {/if}
</div>

<style>
  .roomlist-container {
    padding: 20px;
  }
  
  .data-display {
    margin-top: 20px;
    padding: 10px;
    background-color: #f0f0f0;
    border-radius: 4px;
  }
  
  pre {
    white-space: pre-wrap;
    word-wrap: break-word;
  }
</style>