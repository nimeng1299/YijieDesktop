<script>
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    let { buttons, room_name } = $props();
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
    function leaveRoom() {
        invoke("request_leave_room", {});
    }
    function requestChessStatistics() {
        invoke("request_chess_statistics", { roomName: room_name });
    }
    function requestChessRule() {
        invoke("request_chess_rule", { roomName: room_name });
    }
    function customBottomEvent(event) {
        invoke("request_custom_bottom_event", { event: event });
    }
    function changeReply() {
        invoke("change_reply", {});
    }
</script>

<div style="">
    <div class="flex items-center justify-center gap-2 flex-wrap">
        <button class="btn btn-outline btn-accent" onclick={leaveSeat}
            >让座</button
        >
        <button class="btn btn-outline btn-accent" onclick={admitDefeat}
            >认输</button
        >
    </div>
    <div class="flex items-center justify-center gap-2 flex-wrap">
        <button
            class="btn btn-outline btn-accent"
            style="margin-top: 5px;"
            onclick={changeReply}>{reply}</button
        >
        <button
            class="btn btn-outline btn-accent"
            style="margin-top: 5px;"
            onclick={leaveRoom}>离开房间</button
        >
    </div>
    <div class="flex items-center justify-center gap-2 flex-wrap">
        <button
            class="btn btn-outline btn-accent"
            style="margin-top: 5px;"
            onclick={requestChessStatistics}>排行榜单</button
        >
        <button
            class="btn btn-outline btn-accent"
            style="margin-top: 5px;"
            onclick={requestChessRule}>查看规则</button
        >
    </div>
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
