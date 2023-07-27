from deltalake import DeltaTable
import pyarrow.fs as fs

path = "delta_example/"
filesystem = fs.SubTreeFileSystem(path, fs.LocalFileSystem())
dt = DeltaTable(path)
ds = dt.to_pyarrow_dataset(filesystem=filesystem)
print(ds.to_table().to_pandas())