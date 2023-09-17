import React, { useEffect, useRef } from 'react';
import init, { start_game } from 'source';


const WasmCanvas = () => {
    const initialized = useRef(false);
    useEffect(() => {
        if (initialized.current) return;
        let cleanup: (() => void) | null = null;

        const loadWasm = async () => {
            try {
                const game = await initializeGame();
                startGameLoop(game);
                cleanup = setupKeyboardControls(game);
            } catch (err: unknown) {
                console.error(`Unexpected error in loadWasm. [Message: ${err}]`);
            }
        };

        loadWasm();
        // to prevent re-rendering/calling events twice
        initialized.current = true;

        // Cleanup logic here, this will run when the component unmounts
        return () => {
            if (cleanup) {
                cleanup();
            }
        };
    }, []);

    const initializeGame = async () => {
        await init();
        const game = start_game();
        game.draw_board();
        return game;
    };

    const startGameLoop = (game: any) => {
        let last = Date.now();
        const delay = 3000;
        let animationFrameId: number;

        const mainLoop = () => {
            if (Date.now() - last > delay) {
                game.tick();
                console.log("GameT: Tick Tick!!");
                last = Date.now();
            }
            animationFrameId = requestAnimationFrame(mainLoop);
        };

        animationFrameId = requestAnimationFrame(mainLoop);
        return () => {
            cancelAnimationFrame(animationFrameId);
        };
    };

    const setupKeyboardControls = (game: any) => {
        const keyboardControls = (event: KeyboardEvent) => {
            // TODO: fix deprecated keyCode attr
            if (event.keyCode === 37) {
                console.log("move left");
                game.move_left();
            } else if (event.keyCode === 82) {
                console.log("--rotate boiii");
                game.rotate();
            } else if (event.keyCode === 39) {
                console.log("move left");
                game.move_right();
            } else if (event.keyCode === 40) {
                game.move_down();
            }
        };

        document.addEventListener('keydown', keyboardControls, false);

        return () => {
            document.removeEventListener('keydown', keyboardControls, false);
        };
    };


    return (
        <React.StrictMode>
             <canvas id="myCanvas" width="2000" height="2000"></canvas>
        </React.StrictMode>
       
    );
};

export default WasmCanvas;
