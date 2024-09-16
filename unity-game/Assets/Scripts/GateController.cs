using UnityEngine;

public class GateController : MonoBehaviour
{
    private static readonly int OpenDoor = Animator.StringToHash("open_door");
    
    private Animator _animator;
    private Collider2D _coll;
    
    private bool _opened;
    
    void Start()
    {
        _animator = GetComponent<Animator>();
        _coll = GetComponent<Collider2D>();
    }

    void Update()
    {
        if (_opened)
        {
            _animator.SetTrigger(OpenDoor);
            _coll.enabled = false;
        }
    }
}
