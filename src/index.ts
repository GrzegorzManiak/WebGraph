
import init, {
    Graph,
    DataPoint,
    GraphInitiator,
    LineGraph,
    Point,
    Line
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


    // -- Create a new line
    const line = new Line("#000000", 1.0);
    line.set_point(new DataPoint(10, 1));
    line.set_point(new DataPoint(20, 2));
    line.set_point(new DataPoint(30, 3));
    line.set_point(new DataPoint(40, 4));
    line.set_point(new DataPoint(50, 5));

    // -- Add the line to the graph
    graph.add_line(line);

    // -- Draw the graph
    graph.draw();
}
