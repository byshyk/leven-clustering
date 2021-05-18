from leven_clustering import clustering


def test_clustering() -> None:
    labels = clustering(['aba', 'bbb', 'cbb', 'aad', 'abc'], 0.4, 'average')
    assert labels == [1, 0, 0, 1, 2]  # TODO write comparator for labels (e.g. [1,0,1] == [0,1,0])
