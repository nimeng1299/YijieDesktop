<script lang="ts">
    import "../app.css";
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";
    import TabContent from "./TabContent.svelte";
    import { show } from "@tauri-apps/api/app";
    import CloseButton from "./content/CloseButton.svelte";
    import MaximizeButton from "./content/MaximizeButton.svelte";
    import MinimizeButton from "./content/MinimizeButton.svelte";
    import ReplyButton from "./content/ReplyButton.svelte";

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
    <div class="title-bar">
        <div class="title-content">逆界</div>
        <div class="title-content" data-tauri-drag-region></div>
        <div class="title-content">
            <ReplyButton />
            <MinimizeButton />
            <MaximizeButton />
            <CloseButton />
        </div>
    </div>
    <div style="main-div  ">
        <TabContent />
    </div>
    <div id="toast-container" class="toast toast-end"></div>
</main>

<style>
    .title-bar {
        margin: 10px;
        display: grid;
        grid-template-columns: auto 1fr auto; /* 关键：左右自动宽度，中间占剩余空间 */
    }
    .title-content {
        height: 25px;
    }
    .main-div {
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
