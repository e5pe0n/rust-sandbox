mod add;
mod rm;

use add::AddOptions;
use rm::RmOptions;
pub enum SubcmdOptions {
    Add(AddOptions),
    Rm(RmOptions),
}
