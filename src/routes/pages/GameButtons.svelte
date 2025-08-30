<script>
    import { invoke } from "@tauri-apps/api/core";

    let {tabId, buttons} = $props();

    function leaveSeat() {
        invoke("request_leave_seat", {tabId: tabId});
    }
    function admitDefeat() {
        invoke("request_admit_defeat", {tabId: tabId});
    }
    function customBottomEvent(event) {
        invoke("request_custom_bottom_event", {tabId: tabId, event: event});
    }

</script>

<div style="">
    <div class="flex items-center justify-between margin-top: 10px;">
        <button class="btn btn-outline btn-accent" onclick={leaveSeat}>让座</button>
        <button class="btn btn-outline btn-accent" onclick={admitDefeat}>认输</button>
    </div>
    <button class="btn btn-outline btn-accent" style="margin-top: 5px;">离开房间</button>
    <div style="margin-top: 10px;">
        {#each buttons as button}
            <button class="btn btn-outline btn-info" style="margin-top: 10px;" onclick={customBottomEvent.bind(null, button)}>{button}</button>
        {/each}
    </div>
</div>