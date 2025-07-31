
/**
 * Snake namespace
 * made by me
 */
export namespace Snake {
    export interface SnakeNode {
        index: number,                  // node number
        vector: number,                 // it will define direction of node if top, bottom, left or right (0: bottom, 1: right, 2: top, 3: left)
        color: string,                  // node color (black, white, red, green, blue, gray, pink, purple ...)
    }

    export interface Snake {
        node_count: number,             // snake node count including header and tail, it must be greater than 2
        nodes: Array<SnakeNode>,        // snake nodes
        one_color: boolean,             // color mode, 1 = one, 0 = each
        x: number,                      // x position of snake (is related cell)
        y: number                       // y position of snake (is related cell)
    }

    // snake path
    export class SnakePath {
        private _snakePath: Array<{x: number, y: number, v: number}> = [];

        // constructor
        constructor (other?: SnakePath) {
            if (other) {
                this._snakePath = [...other.get()];
            }
        }

        // get all nodes
        public get(): Array<{x: number, y: number, v: number}> {
            return this._snakePath;
        }

        // add new node
        public push(x: number, y: number, v: number): void {
            this._snakePath.push({x, y, v});
        }

        // remove a node if snake passed one.
        public pop(): void {
            this._snakePath.pop();
        }

        public empty(): boolean {
            return this._snakePath.length === 0;
        }

        public count(): number {
            return this._snakePath.length;
        }

        public clear(): void {
            this._snakePath = [];
        }
    }

    // create snake function
    export function createSnake(node_count: number, x?: number, y?: number): Snake {
        let nodes = []
        for (let i = 0; i < node_count; i++) {
            nodes.push({
                index: i,
                vector: 0,
                color: 'black',
            })
        }

        return {
            node_count: node_count,
            nodes: nodes,
            one_color: true,
            x: x ? x : 0,
            y: y ? y : 0
        }
    }

    // set snake one color
    export function setSnakeOneColor(snake: Snake, one_color: boolean, color: string): Snake {
        for (let i = 0; i < snake.nodes.length; i++) {
            snake.nodes[i].color = color
        }
        return {
            node_count: snake.node_count,
            nodes: snake.nodes,
            one_color: one_color,
            x: snake.x,
            y: snake.y
        }
    }

    // set snake each color for all nodes
    export function setSnakeEachColor(snake: Snake, one_color: boolean, color: string): Snake {
        for (let i = 0; i < snake.nodes.length; i++) {
            snake.nodes[i].color = color
        }
        return {
            node_count: snake.node_count,
            nodes: snake.nodes,
            one_color: one_color,
            x: snake.x,
            y: snake.y
        }
    }

    // get snake node item
    export function getSnakeNode(snake: Snake, index: number): SnakeNode {
        return snake.nodes[index];
    }

    // set snake aspect
    export function setSnakeVectors(snake: Snake, vectors: Array<number>): Snake {
        for (let i = 0; i < snake.node_count; i++) {
            snake.nodes[i].vector = vectors[i];
        }
        return {
            node_count: snake.node_count,
            nodes: snake.nodes,
            one_color: snake.one_color,
            x: snake.x,
            y: snake.y
        }
    }

    // move snake
    export function move(snake: Snake, vector?: number): Snake {
        if (vector === undefined || vector === null) {
            vector = snake.nodes[0].vector;
        }
        if (vector === 0) {
            snake.y++;
        } else if (vector === 1) {
            snake.x++;
        } else if (vector === 2) {
            snake.y--;
        } else if (vector === 3) {
            snake.x--;
        }

        snake.nodes[snake.node_count - 1].vector = snake.nodes[snake.node_count - 3].vector;
        for (let i = snake.node_count - 2; i >= 0; i--) { //except for tail
            if (i > 0) {
                snake.nodes[i].vector = snake.nodes[i - 1].vector;
            } else if (i === 0) {
                snake.nodes[i].vector = vector;
            }
        }
        return {
            node_count: snake.node_count,
            nodes: snake.nodes,
            one_color: snake.one_color,
            x: snake.x,
            y: snake.y
        }
    }

    // get header vector
    export function headerVector(snake: Snake): number {
        return snake.nodes[0].vector;
    }
}

export default Snake;
