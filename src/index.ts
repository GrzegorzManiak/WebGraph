
import init, {
    DataPoint,
    GraphInitiator,
    LineGraph,
    Point,
    Line,
    XYAxis,
    Label,
    DashStyle,
} from './wasm/mandelbrot';
init().then(() => main());

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
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
    let line = new Line("#000000", 1.0);
    line.set_point(new DataPoint(1, 0));
    line.set_point(new DataPoint(2, 10));
    line.set_point(new DataPoint(3, -25));
    line.set_point(new DataPoint(4, 0));
    line.set_point(new DataPoint(5, 0));

    graph.add_line(line);
    graph.draw();


    let line2 = new Line(
        "#FF0000", 2.0,
        Label.defualt_graph_label(""),
        new DashStyle(true, 5, 5),
    );
    line2.set_point(new DataPoint(1, 0));
    line2.set_point(new DataPoint(2, 15));
    line2.set_point(new DataPoint(3, -15));
    line2.set_point(new DataPoint(4, 3));
    line2.set_point(new DataPoint(5, 16));

    graph.add_line(line2);
    graph.draw();


    line2.set_point(new DataPoint(1, 0));
}
