#[doc = "Register `DCR1` reader"]
pub struct R(crate::R<DCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR1` writer"]
pub struct W(crate::W<DCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR1_SPEC>;
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
impl From<crate::W<DCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKMODE` reader - Mode 0 / mode 3"]
pub type CKMODE_R = crate::BitReader<bool>;
#[doc = "Field `CKMODE` writer - Mode 0 / mode 3"]
pub type CKMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR1_SPEC, bool, O>;
#[doc = "Field `FRCK` reader - Free running clock"]
pub type FRCK_R = crate::BitReader<bool>;
#[doc = "Field `FRCK` writer - Free running clock"]
pub type FRCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR1_SPEC, bool, O>;
#[doc = "Field `DLYBYP` reader - Delay block bypass"]
pub type DLYBYP_R = crate::BitReader<bool>;
#[doc = "Field `DLYBYP` writer - Delay block bypass"]
pub type DLYBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR1_SPEC, bool, O>;
#[doc = "Field `CSHT` reader - Chip-select high time"]
pub type CSHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSHT` writer - Chip-select high time"]
pub type CSHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `DEVSIZE` reader - Device size"]
pub type DEVSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEVSIZE` writer - Device size"]
pub type DEVSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `MTYP` reader - Memory type"]
pub type MTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MTYP` writer - Memory type"]
pub type MTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free running clock"]
    #[inline(always)]
    pub fn frck(&self) -> FRCK_R {
        FRCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Delay block bypass"]
    #[inline(always)]
    pub fn dlybyp(&self) -> DLYBYP_R {
        DLYBYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Chip-select high time"]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Device size"]
    #[inline(always)]
    pub fn devsize(&self) -> DEVSIZE_R {
        DEVSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Memory type"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<0> {
        CKMODE_W::new(self)
    }
    #[doc = "Bit 1 - Free running clock"]
    #[inline(always)]
    #[must_use]
    pub fn frck(&mut self) -> FRCK_W<1> {
        FRCK_W::new(self)
    }
    #[doc = "Bit 3 - Delay block bypass"]
    #[inline(always)]
    #[must_use]
    pub fn dlybyp(&mut self) -> DLYBYP_W<3> {
        DLYBYP_W::new(self)
    }
    #[doc = "Bits 8:10 - Chip-select high time"]
    #[inline(always)]
    #[must_use]
    pub fn csht(&mut self) -> CSHT_W<8> {
        CSHT_W::new(self)
    }
    #[doc = "Bits 16:20 - Device size"]
    #[inline(always)]
    #[must_use]
    pub fn devsize(&mut self) -> DEVSIZE_W<16> {
        DEVSIZE_W::new(self)
    }
    #[doc = "Bits 24:26 - Memory type"]
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<24> {
        MTYP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr1](index.html) module"]
pub struct DCR1_SPEC;
impl crate::RegisterSpec for DCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr1::R](R) reader structure"]
impl crate::Readable for DCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr1::W](W) writer structure"]
impl crate::Writable for DCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCR1 to value 0"]
impl crate::Resettable for DCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
