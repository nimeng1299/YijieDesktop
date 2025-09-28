<script lang="ts">
    import "../app.css";
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";
    import TabContent from "./TabContent.svelte";
    import { show } from "@tauri-apps/api/app";

    invoke("init_app");

    function showToast(message, type = "success") {
        console.log("toast: ", message, type);
        const toastContainer = document.getElementById("toast-container");
        const toast = document.createElement("div");
        toast.className = `alert alert-${type}`;
        toast.innerHTML = `<span>${message}</span>`;
        toastContainer.appendChild(toast);

        // Auto-remove after 3 seconds
        setTimeout(() => {
            toast.remove();
        }, 3000);
    }

    listen("show_toast", (event) => {
        const [message, type] = event.payload;
        console.log("show_toast: ", message, type);
        showToast(message, type);
    });

    // 监听change_to_hall事件
    listen("login_success", (event) => {
        invoke("need_show_toast", {
            message: "登录成功",
            toastType: "success",
        });
    });
</script>

<main>
    <div style="main-div display: flex; align-items: center; ">
        <TabContent />
    </div>
    <div id="toast-container" class="toast toast-end"></div>
</main>

<style>
    .main-div {
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
