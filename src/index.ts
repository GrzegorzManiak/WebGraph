
import init, {
    DataPoint,
    Padding,
    LineGraph,
    Point,
    Line,
    XYAxis,
    Label,
    DashStyle,
    Grid,
} from './wasm/mandelbrot';
init().then(() => main());

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
    console.info('WASM Bindings loaded');

    // -- Get the elm with the id 'main'
    const canvas = document.getElementById('main') as HTMLCanvasElement;
    console.info('Canvas element found', canvas);

    // -- Create a new graph
    const graph = new LineGraph(
        canvas, 
        Grid.default(),
        XYAxis.default(),
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
    const pad = new Padding(10, 0, 0, 10);

    graph.set_label("Val1", 1, pad);
    graph.set_label("Val2", 2, pad);
    graph.set_label("Val3", 3, pad);
    graph.set_label("Val4", 4, pad);
    graph.set_label("Val5", 5, pad);


    // -- Create a new line
    let line = new Line("#ff6384", 3.5);
    line.set_point(new DataPoint(1, 5));
    line.set_point(new DataPoint(2, 10));
    line.set_point(new DataPoint(3, 5));
    line.set_point(new DataPoint(4, 10));
    line.set_point(new DataPoint(5, 5));

    graph.add_line(line);
    graph.draw();


    let line2 = new Line(
        "#36a2eb", 3.5,
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
}
