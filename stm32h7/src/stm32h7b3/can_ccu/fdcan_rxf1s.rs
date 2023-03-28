#[doc = "Register `FDCAN_RXF1S` reader"]
pub struct R(crate::R<FDCAN_RXF1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF1S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RXF1S` writer"]
pub struct W(crate::W<FDCAN_RXF1S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FDCAN_RXF1S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXF1S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F1FL` reader - Rx FIFO 1 Fill Level"]
pub type F1FL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1FL` writer - Rx FIFO 1 Fill Level"]
pub type F1FL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF1S_SPEC, u8, u8, 7, O>;
#[doc = "Field `F1GI` reader - Rx FIFO 1 Get Index"]
pub type F1GI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1GI` writer - Rx FIFO 1 Get Index"]
pub type F1GI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF1S_SPEC, u8, u8, 7, O>;
#[doc = "Field `F1PI` reader - Rx FIFO 1 Put Index"]
pub type F1PI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1PI` writer - Rx FIFO 1 Put Index"]
pub type F1PI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF1S_SPEC, u8, u8, 7, O>;
#[doc = "Field `F1F` reader - Rx FIFO 1 Full"]
pub type F1F_R = crate::BitReader<bool>;
#[doc = "Field `F1F` writer - Rx FIFO 1 Full"]
pub type F1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_RXF1S_SPEC, bool, O>;
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub type RF1L_R = crate::BitReader<bool>;
#[doc = "Field `RF1L` writer - Rx FIFO 1 Message Lost"]
pub type RF1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_RXF1S_SPEC, bool, O>;
#[doc = "Field `DMS` reader - Debug Message Status"]
pub type DMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMS` writer - Debug Message Status"]
pub type DMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF1S_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Rx FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn f1fl(&mut self) -> F1FL_W<0> {
        F1FL_W::new(self)
    }
    #[doc = "Bits 8:14 - Rx FIFO 1 Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn f1gi(&mut self) -> F1GI_W<8> {
        F1GI_W::new(self)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn f1pi(&mut self) -> F1PI_W<16> {
        F1PI_W::new(self)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full"]
    #[inline(always)]
    #[must_use]
    pub fn f1f(&mut self) -> F1F_W<24> {
        F1F_W::new(self)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> RF1L_W<25> {
        RF1L_W::new(self)
    }
    #[doc = "Bits 30:31 - Debug Message Status"]
    #[inline(always)]
    #[must_use]
    pub fn dms(&mut self) -> DMS_W<30> {
        DMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Rx FIFO 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf1s](index.html) module"]
pub struct FDCAN_RXF1S_SPEC;
impl crate::RegisterSpec for FDCAN_RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf1s::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF1S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf1s::W](W) writer structure"]
impl crate::Writable for FDCAN_RXF1S_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF1S to value 0"]
impl crate::Resettable for FDCAN_RXF1S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
