<script>
    import { createEventDispatcher } from "svelte";

    let { datas, title } = $props();
    let checked = $state(0);

    const dispath = createEventDispatcher();

    function check(index) {
        checked = index;

        console.log("checked: ", index);
        dispath("chenge-index", { index: index });
    }

    function deleted(index) {
        console.log("deleted: ", index);
        dispath("delete-index", { index: index });
    }
</script>

<div class="reply-list-div">
    <table class="reply-list table table-pin-rows">
        <!-- head -->
        <thead>
            <tr>
                <th>{title}</th>
                <th></th>
            </tr>
        </thead>
        <tbody>
            {#each datas as item, index}
                <tr class={index === checked ? "bg-base-200" : ""}>
                    <td onclick={() => check(index)}>第 {index} 步</td>
                    <th>
                        <button
                            class="btn btn-ghost btn-xs"
                            onclick={() => deleted(index)}>删除</button
                        >
                    </th>
                </tr>
            {/each}
        </tbody>
    </table>
</div>

<style>
    .reply-list-div {
        height: 100%;
        overflow-y: auto; /* 添加垂直滚动条 */
    }
    .reply-list {
        height: 100%;
    }
</style>
