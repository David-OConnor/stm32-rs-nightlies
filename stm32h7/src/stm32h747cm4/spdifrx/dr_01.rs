#[doc = "Register `DR_01` reader"]
pub struct R(crate::R<DR_01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PE` reader - Parity Error bit"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `V` reader - Validity bit"]
pub type V_R = crate::BitReader<bool>;
#[doc = "Field `U` reader - User bit"]
pub type U_R = crate::BitReader<bool>;
#[doc = "Field `C` reader - Channel Status bit"]
pub type C_R = crate::BitReader<bool>;
#[doc = "Field `PT` reader - Preamble Type"]
pub type PT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DR` reader - Data value"]
pub type DR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0 - Parity Error bit"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Validity bit"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User bit"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Status bit"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Preamble Type"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Data value"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Data input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr_01](index.html) module"]
pub struct DR_01_SPEC;
impl crate::RegisterSpec for DR_01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr_01::R](R) reader structure"]
impl crate::Readable for DR_01_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DR_01 to value 0"]
impl crate::Resettable for DR_01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
