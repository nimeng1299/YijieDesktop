<script>
  import { createEventDispatcher } from 'svelte';
  
  let ipAddress = $state("");
  let username = $state("");
  
  function handleLogin() {
    // 这里可以添加登录逻辑
    console.log('IP地址:', ipAddress);
    console.log('用户名:', username);
    // 你可以在这里添加实际的登录处理代码
  }
</script>

<div class="login-container">
  <div class="form-group">
    <legend>IP地址:</legend>
    <input 
      type="text" 
      id="ipAddress" 
      class="input input-bordered w-full max-w-xs" 
      placeholder="请输入服务器IP地址，留空为官方服务器" 
      bind:value={ipAddress}
    />
  </div>
  <div class="form-group mt-4">
    <legend>用户名:</legend>
    <input 
      type="text" 
      id="username" 
      class="input input-bordered w-full max-w-xs" 
      placeholder="请输入用户名" 
      bind:value={username}
    />
  </div>
  <div class="form-group mt-6">
    <button 
      class="btn btn-soft btn-primary" 
      onclick={handleLogin}
    >
      登录
    </button>
  </div>
</div>

<style>
  .login-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100vh;
    padding: 20px;
  }
  
  .form-group {
    display: flex;
    flex-direction: column;
    width: 100%;
    max-width: 300px;
  }
  
  label {
    margin-bottom: 0.5rem;
    font-weight: bold;
  }
</style>