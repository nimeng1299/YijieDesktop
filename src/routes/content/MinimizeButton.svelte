<script>
    import { getCurrentWindow } from "@tauri-apps/api/window";

    function clicked() {
        getCurrentWindow().minimize();
    }
</script>

<button class="btn btn-ghost title-btn" onclick={clicked}
    ><svg
        t="1759245780955"
        class="icon"
        viewBox="0 0 1024 1024"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        p-id="1219"
        width="85%"
        height="85%"
        ><path
            d="M853.333333 480a32 32 0 0 1 0 64H170.666667a32 32 0 0 1 0-64h682.666666z"
            fill="#101010"
            p-id="1220"
        ></path></svg
    ></button
>

<style>
    .title-btn {
        height: 100%;
    }
</style>
