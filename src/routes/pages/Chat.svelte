<script>
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    let messages = $state([]);
    let input_msg = $state(""); // 用于绑定 input 内容
    let scrollContainer;

    listen("change_mode", (event) => {
        messages = [];
        input_msg = "";
    });

    listen("push_chat_message", (event) => {
        const [id, name, message] = event.payload;
        messages.push({
            id: id,
            name: name,
            message: message,
        });
        if (scrollContainer) {
            scrollContainer.scrollTop = scrollContainer.scrollHeight;
        }
    });

    function send_message(msg) {
        if (msg.trim() !== "") {
            invoke("send_message", { msg }).then(() => {
                input_msg = "";
            });
        }
    }
</script>

<div class=" flex flex-col h-[100%]">
    <div class=" flex-1 min-h-0 overflow-y-auto" bind:this={scrollContainer}>
        <table class="table">
            <!-- head -->
            <thead>
                <tr>
                    <th class="w-12">id</th>
                    <!-- 固定宽度列 -->
                    <th class="w-12">玩家名称</th>
                    <!-- 固定宽度列 -->
                    <th class="w-full">消息内容</th>
                    <!-- 占满剩余空间 -->
                </tr></thead
            >
            <tbody>
                {#each messages as msg}
                    <!-- row 1 -->
                    <tr>
                        <th>{msg.id}</th>
                        <td>{msg.name}</td>
                        <td>{msg.message}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
    <div class="p-2 w-full flex-shrink-0">
        <div class="send-div join mt-2">
            <input
                class="input join-item"
                bind:value={input_msg}
                placeholder="发送消息..."
            />
            <button
                class="btn join-item rounded-r-full"
                onclick={() => send_message(input_msg)}>发送</button
            >
        </div>
    </div>
</div>

<style>
    .send-div {
        width: 100%;
    }
</style>
