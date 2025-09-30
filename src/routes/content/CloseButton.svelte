<script>
    import { getCurrentWindow } from "@tauri-apps/api/window";

    function clicked() {
        getCurrentWindow().close();
    }
</script>

<button class="btn btn-ghost title-btn" onclick={clicked}
    ><svg
        t="1759244690670"
        class="icon"
        viewBox="0 0 1024 1024"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        p-id="914"
        width="85%"
        height="85%"
        ><path
            d="M199.722667 802.474667L802.474667 199.765333a39.936 39.936 0 0 1 56.490666 56.490667L256.213333 859.008a39.936 39.936 0 1 1-56.490666-56.533333z"
            fill="#101010"
            p-id="915"
        ></path><path
            d="M858.965333 802.474667l2.773334 3.072a39.978667 39.978667 0 0 1-56.192 56.234666l-3.072-2.773333L199.722667 256.256a39.978667 39.978667 0 0 1 56.490666-56.490667l602.752 602.752z"
            fill="#101010"
            p-id="916"
        ></path></svg
    ></button
>

<style>
    .title-btn {
        height: 100%;
    }
</style>
