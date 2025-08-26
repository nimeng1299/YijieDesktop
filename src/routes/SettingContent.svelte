<script>
  export let tabId;
  export let instance = {};
  
  // 使用instance对象来维护独立的状态
  let inputValue = instance.value || '';
  
  function handleInput(event) {
    inputValue = event.target.value;
    // 更新instance对象中的值
    instance.value = inputValue;
  }
</script>

<div class="p-4">
  <h2 class="text-xl font-bold mb-4">设置页面 (Tab {tabId})</h2>
  <input 
    type="text" 
    placeholder="设置内容" 
    class="input input-bordered w-full max-w-xs"
    value={inputValue}
    on:input={handleInput}
  />
  <p class="mt-4">这是设置页面的内容，每个设置标签页都有独立的状态。</p>
</div>

<style>
  /* Add any additional styles here */
</style>