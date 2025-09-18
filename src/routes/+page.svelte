<script lang="ts">
    import "../app.css";
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";
    import TabContent from "./TabContent.svelte";
    import { show } from "@tauri-apps/api/app";

    invoke("init_app");
    let tabs = $state([{ id: 0, name: "首页", mode: "main" }]);

    function addTab() {
        invoke("add_tab_main");
    }

    function closeTab(id: number) {
        invoke("close_tab", { tabId: id });
    }

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

    listen("change_tabs", (event) => {
        tabs = event.payload.tabs;
        console.log("change_tabs: ", event);
    });

    listen("show_toast", (event) => {
        const [message, type] = event.payload;
        console.log("show_toast: ", message, type);
        showToast(message, type);
    });

    // 监听change_to_hall事件
    listen("login_success", (event) => {
        const [tabId_, name] = event.payload;
        tabs = tabs.map((tab) => {
            if (tab.id === tabId_) {
                tab.name = name;
            }
            return tab;
        });
    });
</script>

<main>
    <div class="tabs tabs-border" style="display: flex; align-items: center; ">
        {#each tabs as tab}
            <label class="tab">
                <input
                    type="radio"
                    name="my_tabs_1"
                    checked={tabs.length === 1}
                />
                {"(" + tab.id + ")" + tab.name}
                <button
                    class="btn btn-xs btn-circle btn-ghost ml-2"
                    style="z-index: 99999;"
                    aria-label="关闭标签页"
                    onclick={() => {
                        closeTab(tab.id);
                    }}
                >
                    ✕
                </button>
            </label>
            <div class="tab-content bg-base-100 w-full h-full">
                {#if tab.mode === "main"}
                    <TabContent tabId={tab.id} />
                {/if}
            </div>
        {/each}
        <button
            class="btn btn-xs btn-circle btn-ghost ml-2 control-btn"
            style="z-index: 99999;"
            aria-label="关闭标签页"
            onclick={() => {
                addTab();
            }}
        >
            <span class="text-xl">+</span>
        </button>
    </div>
    <div id="toast-container" class="toast toast-end"></div>
</main>

<style>
    .control-btn {
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
