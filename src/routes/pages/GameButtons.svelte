<script>
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    let { buttons } = $props();
    let reply = $state("开始录制");

    listen("is_start_reply", (event) => {
        const is_start = event.payload;
        if (is_start) {
            reply = "结束录制";
        } else {
            reply = "开始录制";
        }
    });

    function leaveSeat() {
        invoke("request_leave_seat", {});
    }
    function admitDefeat() {
        invoke("request_admit_defeat", {});
    }
    function LeaveRoom() {
        invoke("request_leave_room", {});
    }
    function customBottomEvent(event) {
        invoke("request_custom_bottom_event", { event: event });
    }
    function changeReply() {
        invoke("change_reply", {});
    }
</script>

<div style="">
    <div class="flex items-center justify-between margin-top: 10px;">
        <button class="btn btn-outline btn-accent" onclick={leaveSeat}
            >让座</button
        >
        <button class="btn btn-outline btn-accent" onclick={admitDefeat}
            >认输</button
        >
    </div>
    <button
        class="btn btn-outline btn-accent"
        style="margin-top: 5px;"
        onclick={changeReply}>{reply}</button
    >
    <button
        class="btn btn-outline btn-accent"
        style="margin-top: 5px;"
        onclick={LeaveRoom}>离开房间</button
    >
    <div style="margin-top: 10px;">
        {#each buttons as button}
            <button
                class="btn btn-outline btn-info"
                style="margin-top: 10px;"
                onclick={customBottomEvent.bind(null, button)}>{button}</button
            >
        {/each}
    </div>
</div>
