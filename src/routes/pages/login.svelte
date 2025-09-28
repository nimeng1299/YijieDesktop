<script>
    import { invoke } from "@tauri-apps/api/core";

    let ipAddress = $state("");
    let username = $state("");

    async function handleLogin() {
        // 调用后端的Rust login函数
        try {
            let login_result = await invoke("login", {
                ip: ipAddress,
                name: username,
            });
            console.log("Login successful:", login_result);
        } catch (error) {
            console.error("Login failed:", error);
        }
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
        <button class="btn btn-soft btn-primary" onclick={handleLogin}>
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
        height: 100%;
        padding: 20px;
        height: 80vh;
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
