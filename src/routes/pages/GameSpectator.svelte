<script>
    let {room_name, spectator} = $props();
</script>

<div>
    <p><strong>{room_name}</strong></p>
    {#if spectator.length > 0 && spectator[0] !== "-1"}
        <p> {spectator}</p>
    {/if}
</div>