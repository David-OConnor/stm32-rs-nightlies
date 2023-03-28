#[doc = "Register `ETH_MACHWF3R` reader"]
pub struct R(crate::R<ETH_MACHWF3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACHWF3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACHWF3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACHWF3R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NRVF` reader - Number of Extended VLAN Tag Filters Enabled This field indicates the Number of Extended VLAN Tag Filters selected: 110 to 111: Reserved, must not be used"]
pub type NRVF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CBTISEL` reader - Queue/Channel based VLAN tag insertion on Tx enable This bit is set to 1 when the Enable Queue/Channel based VLAN tag insertion on Tx feature is selected."]
pub type CBTISEL_R = crate::BitReader<bool>;
#[doc = "Field `DVLAN` reader - Double VLAN processing enable This bit is set to 1 when Double VLAN processing is enabled."]
pub type DVLAN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - Number of Extended VLAN Tag Filters Enabled This field indicates the Number of Extended VLAN Tag Filters selected: 110 to 111: Reserved, must not be used"]
    #[inline(always)]
    pub fn nrvf(&self) -> NRVF_R {
        NRVF_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Queue/Channel based VLAN tag insertion on Tx enable This bit is set to 1 when the Enable Queue/Channel based VLAN tag insertion on Tx feature is selected."]
    #[inline(always)]
    pub fn cbtisel(&self) -> CBTISEL_R {
        CBTISEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Double VLAN processing enable This bit is set to 1 when Double VLAN processing is enabled."]
    #[inline(always)]
    pub fn dvlan(&self) -> DVLAN_R {
        DVLAN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "HW feature 3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_machwf3r](index.html) module"]
pub struct ETH_MACHWF3R_SPEC;
impl crate::RegisterSpec for ETH_MACHWF3R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_machwf3r::R](R) reader structure"]
impl crate::Readable for ETH_MACHWF3R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACHWF3R to value 0x20"]
impl crate::Resettable for ETH_MACHWF3R_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
