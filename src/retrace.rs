use anyhow::Result;
use proguard::ProguardMapper;
use tokio::fs;

use crate::dirs::DIRS;

pub async fn retrace(stacktrace: &str) -> Result<String> {
    let mapping = fs::read_to_string(DIRS.mapping_file()).await?;
    ProguardMapper::from(mapping.as_ref())
        .remap_stacktrace(stacktrace)
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use proguard::ProguardMapper;

    const MAPPING: &str = include_str!("../mapping.txt");

    #[test]
    fn retrace_class() {
        let mapper = ProguardMapper::from(MAPPING);

        println!(
            "{}",
            mapper
                .remap_stacktrace(
                    "
    at rocks.dnaka91.reciply.fragment.RecipeFragment$f.onClick(SourceFile:1)
    at android.view.View.performClick(View.java:7393)
    at com.google.android.material.button.MaterialButton.performClick(SourceFile:2)
    at android.view.View.performClickInternal(View.java:7370)
    at android.view.View.access$3700(View.java:815)
    at android.view.View$PerformClick.run(View.java:28508)
    at android.os.Handler.handleCallback(Handler.java:938)
    at android.os.Handler.dispatchMessage(Handler.java:99)
    at android.os.Looper.loopOnce(Looper.java:201)
    at android.os.Looper.loop(Looper.java:288)
    at android.app.ActivityThread.main(ActivityThread.java:7729)
    at java.lang.reflect.Method.invoke(Native Method)
    at com.android.internal.os.RuntimeInit$MethodAndArgsCaller.run(RuntimeInit.java:548)
    at com.android.internal.os.ZygoteInit.main(ZygoteInit.java:974)"
                )
                .unwrap()
        );
    }
}
