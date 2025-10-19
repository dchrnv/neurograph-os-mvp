"""
SQLAlchemy models for NeuroGraph OS persistence layer.
Supports Token, Graph, and Experience entities.
"""

from datetime import datetime
from typing import Optional
from sqlalchemy import (
    Column, Integer, String, Float, Boolean, DateTime, 
    ForeignKey, Index, JSON, LargeBinary, BigInteger
)
from sqlalchemy.orm import declarative_base, relationship
from sqlalchemy.dialects.postgresql import UUID, ARRAY, JSONB
import uuid

Base = declarative_base()


class TokenModel(Base):
    """Token entity persistence model."""
    
    __tablename__ = "tokens"
    __table_args__ = (
        Index('idx_token_coords', 'coord_x', 'coord_y', 'coord_z'),
        Index('idx_token_created', 'created_at'),
        Index('idx_token_flags', 'flags'),
        {'schema': 'tokens'}
    )
    
    # Primary Key
    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    
    # Binary Data
    binary_data = Column(LargeBinary(64), nullable=False)  # 64 bytes token format
    
    # Coordinates (8 levels x 3 axes)
    coord_x = Column(ARRAY(Float), nullable=False)  # 8 levels
    coord_y = Column(ARRAY(Float), nullable=False)
    coord_z = Column(ARRAY(Float), nullable=False)
    
    # Metadata
    flags = Column(BigInteger, default=0)
    weight = Column(Float, default=1.0)
    timestamp = Column(BigInteger, nullable=False)  # Unix timestamp in ms
    
    # Additional fields
    token_type = Column(String(50), default='default')
    metadata = Column(JSONB, default=dict)
    
    # Audit fields
    created_at = Column(DateTime, default=datetime.utcnow, nullable=False)
    updated_at = Column(DateTime, default=datetime.utcnow, onupdate=datetime.utcnow)
    
    # Relationships
    connections_from = relationship(
        "ConnectionModel", 
        foreign_keys="ConnectionModel.source_id",
        back_populates="source_token",
        cascade="all, delete-orphan"
    )
    connections_to = relationship(
        "ConnectionModel", 
        foreign_keys="ConnectionModel.target_id",
        back_populates="target_token"
    )
    experiences = relationship("ExperienceEventModel", back_populates="token")
    
    def __repr__(self):
        return f"<TokenModel(id={self.id}, type={self.token_type})>"


class ConnectionModel(Base):
    """Graph connection/edge persistence model."""
    
    __tablename__ = "connections"
    __table_args__ = (
        Index('idx_connection_source', 'source_id'),
        Index('idx_connection_target', 'target_id'),
        Index('idx_connection_type', 'connection_type'),
        Index('idx_connection_weight', 'weight'),
        {'schema': 'graph'}
    )
    
    # Composite Primary Key
    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    source_id = Column(UUID(as_uuid=True), ForeignKey('tokens.tokens.id'), nullable=False)
    target_id = Column(UUID(as_uuid=True), ForeignKey('tokens.tokens.id'), nullable=False)
    
    # Connection Properties
    connection_type = Column(String(50), default='generic')  # spatial, temporal, semantic
    weight = Column(Float, default=1.0)
    decay_rate = Column(Float, default=0.0)
    bidirectional = Column(Boolean, default=False)
    
    # Metadata
    metadata = Column(JSONB, default=dict)
    created_at = Column(DateTime, default=datetime.utcnow, nullable=False)
    updated_at = Column(DateTime, default=datetime.utcnow, onupdate=datetime.utcnow)
    
    # Genetic Algorithm fields
    generation = Column(Integer, default=0)
    fitness = Column(Float, default=0.0)
    
    # Relationships
    source_token = relationship(
        "TokenModel", 
        foreign_keys=[source_id],
        back_populates="connections_from"
    )
    target_token = relationship(
        "TokenModel", 
        foreign_keys=[target_id],
        back_populates="connections_to"
    )
    
    def __repr__(self):
        return f"<ConnectionModel(source={self.source_id}, target={self.target_id}, type={self.connection_type})>"


class GraphSnapshotModel(Base):
    """Graph state snapshot for versioning and rollback."""
    
    __tablename__ = "graph_snapshots"
    __table_args__ = {'schema': 'graph'}
    
    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    generation = Column(Integer, nullable=False)
    fitness = Column(Float, default=0.0)
    
    # Statistics
    node_count = Column(Integer, default=0)
    edge_count = Column(Integer, default=0)
    avg_degree = Column(Float, default=0.0)
    clustering_coefficient = Column(Float, default=0.0)
    
    # Serialized graph state
    graph_data = Column(JSONB, nullable=False)
    metadata = Column(JSONB, default=dict)
    
    created_at = Column(DateTime, default=datetime.utcnow, nullable=False)
    
    def __repr__(self):
        return f"<GraphSnapshot(gen={self.generation}, nodes={self.node_count})>"


class ExperienceEventModel(Base):
    """Experience event persistence model."""
    
    __tablename__ = "experience_events"
    __table_args__ = (
        Index('idx_experience_timestamp', 'timestamp'),
        Index('idx_experience_type', 'event_type'),
        Index('idx_experience_token', 'token_id'),
        Index('idx_experience_priority', 'priority'),
        {'schema': 'experience'}
    )
    
    # Primary Key
    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    
    # Event Properties
    event_type = Column(String(100), nullable=False)
    timestamp = Column(BigInteger, nullable=False)
    
    # Associated Token
    token_id = Column(UUID(as_uuid=True), ForeignKey('tokens.tokens.id'), nullable=True)
    
    # State Information
    state_before = Column(JSONB)
    state_after = Column(JSONB)
    action = Column(JSONB)
    reward = Column(Float, default=0.0)
    
    # Metadata
    metadata = Column(JSONB, default=dict)
    priority = Column(Float, default=1.0)
    
    # Trajectory Information
    trajectory_id = Column(UUID(as_uuid=True), nullable=True)
    sequence_number = Column(Integer, default=0)
    
    created_at = Column(DateTime, default=datetime.utcnow, nullable=False)
    
    # Relationship
    token = relationship("TokenModel", back_populates="experiences")
    
    def __repr__(self):
        return f"<ExperienceEvent(type={self.event_type}, token={self.token_id})>"


class ExperienceTrajectoryModel(Base):
    """Experience trajectory for sequential events."""
    
    __tablename__ = "experience_trajectories"
    __table_args__ = (
        Index('idx_trajectory_start', 'start_time'),
        Index('idx_trajectory_end', 'end_time'),
        {'schema': 'experience'}
    )
    
    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    
    start_time = Column(BigInteger, nullable=False)
    end_time = Column(BigInteger, nullable=True)
    
    event_count = Column(Integer, default=0)
    total_reward = Column(Float, default=0.0)
    
    metadata = Column(JSONB, default=dict)
    created_at = Column(DateTime, default=datetime.utcnow, nullable=False)
    
    def __repr__(self):
        return f"<Trajectory(id={self.id}, events={self.event_count})>"


class SpatialIndexModel(Base):
    """Spatial indexing metadata for performance optimization."""
    
    __tablename__ = "spatial_index"
    __table_args__ = (
        Index('idx_spatial_region', 'region_id'),
        Index('idx_spatial_bounds', 'min_x', 'min_y', 'min_z', 'max_x', 'max_y', 'max_z'),
        {'schema': 'tokens'}
    )
    
    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    region_id = Column(String(100), unique=True, nullable=False)
    
    # Bounding box
    min_x = Column(Float, nullable=False)
    min_y = Column(Float, nullable=False)
    min_z = Column(Float, nullable=False)
    max_x = Column(Float, nullable=False)
    max_y = Column(Float, nullable=False)
    max_z = Column(Float, nullable=False)
    
    token_count = Column(Integer, default=0)
    density = Column(Float, default=0.0)
    
    metadata = Column(JSONB, default=dict)
    updated_at = Column(DateTime, default=datetime.utcnow, onupdate=datetime.utcnow)
    
    def __repr__(self):
        return f"<SpatialIndex(region={self.region_id}, tokens={self.token_count})>"