# raesan-regsitry

A fully open-source, exam-focused question registry for JEE, NEET, and other standardized tests.  
This project provides a structured JSON registry that compiles into a lightweight SQLite database — perfect for powering educational tools, test generators, or learning platforms.

## 🚀 Features

- ✅ Fully open-source and self-hostable
- 📁 Simple JSON structure for easy editing/versioning
- 🧠 Covers exams, subjects, chapters, and questions
- 💾 Compiles into a single portable SQLite database
- 🧪 Ideal for custom test apps and learning platforms

## ⚙️ registry application (linux only)

The registry, other than providing questions, can be used to do a variety of things.

The simplest way to run `raesan-registry` application is using [nix flakes](https://nixos.wiki/wiki/Flakes)

> [!NOTE]: [Nix](https://nix.dev/manual/nix/2.24/introduction) is a extremely easy and extremely complex thing to deal with, so if you try to mess around with it, make sure you understand what you are doing

1. Install `nix` on your system
   ```bash
   sh <(curl -L https://nixos.org/nix/install) --no-daemon
   ```
2. Restart your device.
3. Add the following content to `/etc/nix/nix.conf` file in order to setup flakes

   ```bash
   experimental-features = nix-command flakes
   ```

4. Clone this gitub repository and go into that directory

   ```bash
   git clone https://github.com/raesan-project/registry.git && cd ./registry
   ```

5. Run the following command
   ```bash
   nix run github:raesan-project/registry
   ```

> [!NOTE]: This process can be heavy on your computer depending upon your specs and it can take a considerable amount of time (2-10 mins)

After doing all that you will see something like this in the terminal

```bash
Usage: raesan_registry-bin <COMMAND>

Commands:
  serve-questions
          start a server to render the questions
  generate-database-records
          generate SQLite database records from registry
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help
```

These are the CLI options of the regsitry application.

You can either choose `server-questions` and a web server will start, and you can to your `localhost:8080` in the browser to see all the questions in the registry. The web server watches for any changes in the \_registry folder, so if you make any change in any file of the \_registry, it will me visible on the website in real time.<br/>
In order to start the server just run the following command:

```bash
nix run github:raesan-project/registry serve-questions -- --registry ./_registry
```

Or you can choose `generate-database-records` which can be used to unload the entire registry into an SQLite database file<br/>
In order to do that go through the following steps:

1. Run the following command while being in the same folder as `flake.nix` file
   ```bash
   nix develop
   ```
   > [!NOTE]: The above command will start a simple dev-shell environment which contains all the project dependencies in a simple isolated state. Checkout [nix shell](https://nix.dev/manual/nix/2.18/command-ref/nix-shell) for more information
2. Now run the following command to create a simple `raesan.db` file in the project root (you can name it whatever you want, just change the name in the command before running it)

   ```bash
   diesel setup --database-url raesan.db
   diesel migration run --database-url raesan.db
   ```

   > [!NOTE]
   >
   > This uses [diesel](https://diesel.rs/) to setup the database and run the necessary migration on it.
   > In order to better understand the database structures I highly recommend checking out [this](https://github.com/raesan-project/registry/blob/main/migrations/2025-04-06-090738_base/up.sql), [this](https://github.com/raesan-project/registry/blob/main/src/tables.rs), and [this](https://github.com/raesan-project/registry/blob/main/src/registry/reg_models.rs) file

3. Now run the following command to finally unload the registry onto that database file
   ```bash
   nix run github:raesan-project/registry generate-database-records -- --database raesan.db --registry ./_registry/
   ```
4. Now you have a single SQLite database file that contains all the information about the registry

## 📂 Registry Structure

The basic structure of the registry is as folllows:

```bash
_registry/
├─ exam-1/
│   ├─_index.json
│   └─subject/
│      ├─_index.json
│      ├─chapter_1.json
│      └─chapter_2.json
└─ exam-2/
```

It is a hierarchical data model.

> [!NOTE]: All the paths in the registry must be lowercase and should not contain any special characters or any non-alphanumerical characters, a simple example of a valid path: magnetic-properties-of-matter.json

The `_registry` folder contains all the exams as folders. Those exam folders will contain `_index.json` file and subjects as folders. The `_index.json` is a special file, it tells the registry application important information about the currect directory(which can be exam, subject or a chapter)

The structure of every file(including \_index.json file) in the registry looks like the following:

```json
{
	"_index": {},
	"_children": []
}
```

The `_index: {}` field will contain data about the current folder/file(exam,subject,chapter) and `_children` can contain either of the following: relative paths of the children of that folder or the actual children of the file

Let's take a simple example of [/\_registry/jee-main/\_index.json](https://github.com/raesan-project/registry/blob/main/_registry/jee-main/_index.json) file, the contents of this file looks like this

```json
{
	"_index": {
		"name": "JEE Main"
	},
	"_children": ["./physics", "./chemistry", "./mathematics"]
}
```

The \_index field contains the name of the current exam(JEE Main) and the \_children contains relative links to the various subjects contained in JEE Main.<br/>
The only folders that are linked in the \_index.json file of JEE Main will be regarded as subjects of that exam, so if you just create the folder and forget to add it to \_index.json it will be ignored by the registry applicaton.

Let's take a look at [/\_registry/jee-main/physics/\_index.json](https://github.com/raesan-project/registry/blob/main/_registry/jee-main/physics/_index.json)

```
{
  "_index": {
    "name": "Physics"
  },
  "_children": [
    "./units-and-measurements.json",
    "./vector-algebra.json",
    "./motion-in-a-straight-line.json",
    "./motion-in-a-plane.json",
    "./circular-motion.json",
    "./laws-of-motion.json",
    "./work-power-and-energy.json",
    ...
  ]
}
```

As you can see, the \_index contains information about the current subject(Physics) and \_index.json contains relative links to various chapters contained in JEE-Main/Physics subject.<br/>
The same ignore functionality is valid for this. So if you want a file to not be ignored by registry application, add the relative path to that file here.

Now let's move onto the [/\_registry/jee-main/physics/\capacitor.json](https://github.com/raesan-project/registry/blob/main/_registry/jee-main/physics/capacitor.json)

```
{
  "_index": {
    "name": "Capacitor"
  },
  "_children": [
    {
      "_type": "numerical",
      "body": "A 5 $$\\mu $$F capacitor is charged fully by a 220 V\nsupply. It is then disconnected from the supply\nand is connected in series to another\nuncharged 2.5 $$\\mu $$F capacitor. If the energy\nchange during the charge redistribution is\n$${X \\over {100}}J$$ then value of X to the nearest integer is\n_____.",
      "answer": "4",
      "explanation": "u<sub>i</sub> = $$\\frac{1}{2} $$ $$ \\times $$ 5 $$ \\times $$ 10<sup>-6</sup>$$ \\times $$220\n<br><br>Final common potential\n<br><br>= $$\\frac{220\\times 5+0\\times 2.5}{5+2.5} $$ = 220 $$ \\times $$ $$\\frac{2}{3} $$\n<br><br>u<sub>f</sub> = $$\\frac{1}{2} $$ $$ \\times $$ (5 + 2.5)$$ \\times $$10<sup>-6</sup> $$ \\times $$ $$\\left( 220\\times \\frac{2}{3} \\right)^{2} $$\n<br><br>$$\\Delta $$u = u<sub>i</sub> - u<sub>f</sub>\n<br><br>$$ \\Rightarrow $$ $$\\Delta $$u = –403.33 × 10<sup>–4</sup>\n<br><br>$$ \\Rightarrow $$ –403.33 × 10<sup>–4</sup> = $${X \\over {100}}J$$\n<br><br>$$ \\Rightarrow $$ X = -4.03\n<br><br>Value of X is approximate 4"
    },
    {
      "_type": "single_mcq",
      "body": "<p>A parallel plate capacitor of capacitance 1 µF is charged to a potential difference of 20 V. The distance between plates is 1 µm. The energy density between plates of capacitor is :</p>",
      "options": [
        {
          "key": "A",
          "value": "<p>$1.8 \\times 10^3$ J/m<sup>3</sup></p>"
        },
        {
          "key": "B",
          "value": "<p>$2 \\times 10^2$ J/m<sup>3</sup></p>"
        },
        {
          "key": "C",
          "value": "<p>$2 \\times 10^{-4}$ J/m<sup>3</sup></p>"
        },
        {
          "key": "D",
          "value": "<p>$1.8 \\times 10^5$ J/m<sup>3</sup></p>"
        }
      ],
      "answer": "A",
      "explanation": "<p>The energy density ($u$) between the plates of a capacitor is given by the formula:</p>\n<p>$$ u = \\frac{1}{2} \\varepsilon_0 E^2 $$</p>\n<p>where $\\varepsilon_0$ is the permittivity of free space ($8.85 \\times 10^{-12} \\, \\text{F/m}$) and $E$ is the electric field between the plates. The electric field $E$ is related to the potential difference $V$ and the separation $d$ between the plates by:</p>\n<p>$$ E = \\frac{V}{d} $$</p>\n<p>Given:</p>\n\n<p><p>Capacitance ($C$) = 1 µF = $1 \\times 10^{-6} \\, \\text{F}$</p></p>\n<p><p>Potential Difference ($V$) = 20 V</p></p>\n<p><p>Distance ($d$) = 1 µm = $1 \\times 10^{-6} \\, \\text{m}$</p></p>\n\n<p>First, calculate the electric field $E$:</p>\n<p>$$ E = \\frac{V}{d} = \\frac{20 \\, \\text{V}}{1 \\times 10^{-6} \\, \\text{m}} = 2 \\times 10^7 \\, \\text{V/m} $$</p>\n<p>Now plug this into the formula for energy density:</p>\n<p>$$ u = \\frac{1}{2} \\times 8.85 \\times 10^{-12} \\, \\text{F/m} \\times (2 \\times 10^7 \\, \\text{V/m})^2 $$</p>\n<p>Calculate $u$:</p>\n<p>$$ u = \\frac{1}{2} \\times 8.85 \\times 10^{-12} \\times 4 \\times 10^{14} $$</p>\n<p>$$ u = 2 \\times 8.85 \\times 10^2 $$</p>\n<p>$$ u = 1770 \\, \\text{J/m}^3 $$</p>\n<p>This value is approximately $1.8 \\times 10^3 \\, \\text{J/m}^3$. Therefore, the correct option is:</p>\n<p><strong>Option A</strong>: $1.8 \\times 10^3 \\, \\text{J/m}^3$</p>"
    }
    ...
  ]
}
```

In the \_index field, the similar information about the current chapter(Capacitor) is given. The special difference for this file is the absence of relative file paths in the \_children field.

Since this is a chapter file, the \_children field will contain the actual questions of that chapter in either of the two currently supported formats: Single Correct MCQ, Numericals.<br/>
The `_type` field explains which type(single_mcq,numerical) question it is, `body` contains the actual [latex](https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX?) question body, `options` contains the options of single_mcq question type, `answer` contains the actual answer of the question, the `explanation` contains a basic explanation of the answer provided for the question.

> [!NOTE]: for single_mcq, the `answer` should match the `key` of the correct option in order to properly link with that option, otherwise it will produce faults when reading or checking for this
