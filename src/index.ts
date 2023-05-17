
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
            
    graph.draw();
    console.info('Graph created', graph);
    // -- This just returns a snapshot of the graph
    //    object, editing it will not change the graph
    const raw_graph = graph.get_graph();
    console.info('Raw graph', raw_graph);
    console.log('width', raw_graph.get_width());
    console.log('height', raw_graph.get_height());


    // -- Create a new line
    const line = new Line("#000000", 1.0);
    line.add_point(new DataPoint(10, 1));
    line.add_point(new DataPoint(20, 2));
    line.add_point(new DataPoint(30, 3));
    line.add_point(new DataPoint(40, 4));
    line.add_point(new DataPoint(50, 5));

    // -- Add the line to the graph
    graph.add_line(line);

    // -- Draw the graph
    graph.draw();
}
