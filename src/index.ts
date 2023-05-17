
import init, {
    DataPoint,
    GraphInitiator,
    LineGraph,
    Point,
    Line,
    XYAxis,
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
        ),

        // -- X Axis
        XYAxis.default(),

        // -- Y Axis
        XYAxis.default(),
    );
            
    graph.draw();
    console.info('Graph created', graph);
    // -- This just returns a snapshot of the graph
    //    object, editing it will not change the graph
    const raw_graph = graph.get_graph();
    console.info('Raw graph', raw_graph);
    console.log('width', raw_graph.get_width());
    console.log('height', raw_graph.get_height());  


    // -- Add the lables to the graph
    graph.set_label("Val1", 1);
    graph.set_label("Val2", 2);
    graph.set_label("Val3", 3);
    graph.set_label("Val4", 4);
    graph.set_label("Val5", 5);


    // -- Create a new line
    const line = new Line("#000000", 1.0);
    line.set_point(new DataPoint(1, 0));
    line.set_point(new DataPoint(2, 25));
    line.set_point(new DataPoint(3, -25));
    line.set_point(new DataPoint(4, 50));
    line.set_point(new DataPoint(5, 0));

    // -- Add the line to the graph
    graph.add_line(line);

    // -- Draw the graph
    graph.draw();
}
