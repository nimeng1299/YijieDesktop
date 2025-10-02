<script>
    import GameBoard from "./GameBoard.svelte";
    import GameButtons from "./GameButtons.svelte";
    import GameCard from "./GameCard.svelte";
    import GameSpectator from "./GameSpectator.svelte";

    let { room, game, buttons, countdown, can_move, modes } = $props();
</script>

<div class="game-container" hidden={modes !== "game"}>
    <div class="game-left">
        <div class="gamecard">
            <GameCard
                player={room.black_player}
                score={game.black_score}
                side="black"
                countdown={countdown[0] + Date.now() / 1000}
            />
        </div>
        <div class="gamecard">
            <GameCard
                player={room.white_player}
                score={game.white_score}
                side="white"
                countdown={countdown[1] + Date.now() / 1000}
            />
        </div>
    </div>
    <div class="game-middle">
        <GameBoard {game} {can_move} is_reply={false} />
        <div hidden={game.game_tip === ""}>{game.game_tip}</div>
    </div>
    <div class="game-right">
        <GameSpectator room_name={room.name} spectator={room.spectator}
        ></GameSpectator>
        <GameButtons {buttons}></GameButtons>
    </div>
</div>

<style>
    .game-container {
        display: flex;
        height: 90vh; /* 添加高度以便查看效果 */
        gap: 10px;
    }
    .game-left {
        width: 25%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }
    .gamecard {
        width: 100%; /* 添加宽度以占满 game-left */
    }
    .game-middle {
        width: 60%;
    }
    .game-right {
        width: 15%;
    }
</style>
