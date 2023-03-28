#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOF0` reader - Synchronization overrun event flag"]
pub type SOF0_R = crate::BitReader<bool>;
#[doc = "Field `SOF1` reader - Synchronization overrun event flag"]
pub type SOF1_R = crate::BitReader<bool>;
#[doc = "Field `SOF2` reader - Synchronization overrun event flag"]
pub type SOF2_R = crate::BitReader<bool>;
#[doc = "Field `SOF3` reader - Synchronization overrun event flag"]
pub type SOF3_R = crate::BitReader<bool>;
#[doc = "Field `SOF4` reader - Synchronization overrun event flag"]
pub type SOF4_R = crate::BitReader<bool>;
#[doc = "Field `SOF5` reader - Synchronization overrun event flag"]
pub type SOF5_R = crate::BitReader<bool>;
#[doc = "Field `SOF6` reader - Synchronization overrun event flag"]
pub type SOF6_R = crate::BitReader<bool>;
#[doc = "Field `SOF7` reader - Synchronization overrun event flag"]
pub type SOF7_R = crate::BitReader<bool>;
#[doc = "Field `SOF8` reader - Synchronization overrun event flag"]
pub type SOF8_R = crate::BitReader<bool>;
#[doc = "Field `SOF9` reader - Synchronization overrun event flag"]
pub type SOF9_R = crate::BitReader<bool>;
#[doc = "Field `SOF10` reader - Synchronization overrun event flag"]
pub type SOF10_R = crate::BitReader<bool>;
#[doc = "Field `SOF11` reader - Synchronization overrun event flag"]
pub type SOF11_R = crate::BitReader<bool>;
#[doc = "Field `SOF12` reader - Synchronization overrun event flag"]
pub type SOF12_R = crate::BitReader<bool>;
#[doc = "Field `SOF13` reader - Synchronization overrun event flag"]
pub type SOF13_R = crate::BitReader<bool>;
#[doc = "Field `SOF14` reader - Synchronization overrun event flag"]
pub type SOF14_R = crate::BitReader<bool>;
#[doc = "Field `SOF15` reader - Synchronization overrun event flag"]
pub type SOF15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof2(&self) -> SOF2_R {
        SOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof3(&self) -> SOF3_R {
        SOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof4(&self) -> SOF4_R {
        SOF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof5(&self) -> SOF5_R {
        SOF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof6(&self) -> SOF6_R {
        SOF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof7(&self) -> SOF7_R {
        SOF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof8(&self) -> SOF8_R {
        SOF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof9(&self) -> SOF9_R {
        SOF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof10(&self) -> SOF10_R {
        SOF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof11(&self) -> SOF11_R {
        SOF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof12(&self) -> SOF12_R {
        SOF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof13(&self) -> SOF13_R {
        SOF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof14(&self) -> SOF14_R {
        SOF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof15(&self) -> SOF15_R {
        SOF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt channel status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
