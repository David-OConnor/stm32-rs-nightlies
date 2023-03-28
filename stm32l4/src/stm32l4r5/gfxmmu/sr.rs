#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `B0OF` reader - Buffer 0 overflow flag"]
pub type B0OF_R = crate::BitReader<bool>;
#[doc = "Field `B1OF` reader - Buffer 1 overflow flag"]
pub type B1OF_R = crate::BitReader<bool>;
#[doc = "Field `B2OF` reader - Buffer 2 overflow flag"]
pub type B2OF_R = crate::BitReader<bool>;
#[doc = "Field `B3OF` reader - Buffer 3 overflow flag"]
pub type B3OF_R = crate::BitReader<bool>;
#[doc = "Field `AMEF` reader - AHB master error flag"]
pub type AMEF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Buffer 0 overflow flag"]
    #[inline(always)]
    pub fn b0of(&self) -> B0OF_R {
        B0OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer 1 overflow flag"]
    #[inline(always)]
    pub fn b1of(&self) -> B1OF_R {
        B1OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer 2 overflow flag"]
    #[inline(always)]
    pub fn b2of(&self) -> B2OF_R {
        B2OF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Buffer 3 overflow flag"]
    #[inline(always)]
    pub fn b3of(&self) -> B3OF_R {
        B3OF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master error flag"]
    #[inline(always)]
    pub fn amef(&self) -> AMEF_R {
        AMEF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Graphic MMU status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
