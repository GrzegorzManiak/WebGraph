
import init, {
    Graph,
    DataPoint,
    GraphInitiator,
    LineGraph,
    Point
} from './wasm/mandelbrot';
init().then(() => main());

function main() {
    console.info('WASM Bindings loaded');

    // -- Get the elm with the id 'main'
    const canvas = document.getElementById('main') as HTMLCanvasElement;
    console.info('Canvas element found', canvas);

    // -- Create a new graph
    const graph = new LineGraph(canvas, 
        new GraphInitiator(
            new Point(0, 0),
            new Point(0, 0),
            new Point(0, 0),
        )
    );

    const point = new DataPoint(new Point(0, 0), 10);
    graph.add_point(point);
}
