#[doc = "Register `GCR` reader"]
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCR` writer"]
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTDCEN` reader - LCD-TFT controller enable bit"]
pub type LTDCEN_R = crate::BitReader<bool>;
#[doc = "Field `LTDCEN` writer - LCD-TFT controller enable bit"]
pub type LTDCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `DBW` reader - Dither Blue Width"]
pub type DBW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DGW` reader - Dither Green Width"]
pub type DGW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRW` reader - Dither Red Width"]
pub type DRW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEN` reader - Dither Enable"]
pub type DEN_R = crate::BitReader<bool>;
#[doc = "Field `DEN` writer - Dither Enable"]
pub type DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `PCPOL` reader - Pixel Clock Polarity"]
pub type PCPOL_R = crate::BitReader<bool>;
#[doc = "Field `PCPOL` writer - Pixel Clock Polarity"]
pub type PCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `DEPOL` reader - Not Data Enable Polarity"]
pub type DEPOL_R = crate::BitReader<bool>;
#[doc = "Field `DEPOL` writer - Not Data Enable Polarity"]
pub type DEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `VSPOL` reader - Vertical Synchronization Polarity"]
pub type VSPOL_R = crate::BitReader<bool>;
#[doc = "Field `VSPOL` writer - Vertical Synchronization Polarity"]
pub type VSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
#[doc = "Field `HSPOL` reader - Horizontal Synchronization Polarity"]
pub type HSPOL_R = crate::BitReader<bool>;
#[doc = "Field `HSPOL` writer - Horizontal Synchronization Polarity"]
pub type HSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Dither Blue Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Dither Green Width"]
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Dither Red Width"]
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Not Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<0> {
        LTDCEN_W::new(self)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<16> {
        DEN_W::new(self)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pcpol(&mut self) -> PCPOL_W<28> {
        PCPOL_W::new(self)
    }
    #[doc = "Bit 29 - Not Data Enable Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn depol(&mut self) -> DEPOL_W<29> {
        DEPOL_W::new(self)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<30> {
        VSPOL_W::new(self)
    }
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<31> {
        HSPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](index.html) module"]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcr::R](R) reader structure"]
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcr::W](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCR to value 0x2220"]
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2220;
}
