#[doc = "Register `AHB1RSTR` reader"]
pub struct R(crate::R<AHB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1RSTR` writer"]
pub struct W(crate::W<AHB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1RSTR_SPEC>;
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
impl From<crate::W<AHB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPDMA1RST` reader - GPDMA1 block reset Set and reset by software."]
pub type GPDMA1RST_R = crate::BitReader<bool>;
#[doc = "Field `GPDMA1RST` writer - GPDMA1 block reset Set and reset by software."]
pub type GPDMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `GPDMA2RST` reader - GPDMA2 block reset Set and reset by software."]
pub type GPDMA2RST_R = crate::BitReader<bool>;
#[doc = "Field `GPDMA2RST` writer - GPDMA2 block reset Set and reset by software."]
pub type GPDMA2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `CRCRST` reader - CRC block reset Set and reset by software."]
pub type CRCRST_R = crate::BitReader<bool>;
#[doc = "Field `CRCRST` writer - CRC block reset Set and reset by software."]
pub type CRCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `CORDICRST` reader - CORDIC block reset Set and reset by software."]
pub type CORDICRST_R = crate::BitReader<bool>;
#[doc = "Field `CORDICRST` writer - CORDIC block reset Set and reset by software."]
pub type CORDICRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `FMACRST` reader - FMAC block reset Set and reset by software."]
pub type FMACRST_R = crate::BitReader<bool>;
#[doc = "Field `FMACRST` writer - FMAC block reset Set and reset by software."]
pub type FMACRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `RAMCFGRST` reader - RAMCFG block reset Set and reset by software."]
pub type RAMCFGRST_R = crate::BitReader<bool>;
#[doc = "Field `RAMCFGRST` writer - RAMCFG block reset Set and reset by software."]
pub type RAMCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `ETHRST` reader - ETHRST block reset Set and reset by software"]
pub type ETHRST_R = crate::BitReader<bool>;
#[doc = "Field `ETHRST` writer - ETHRST block reset Set and reset by software"]
pub type ETHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
#[doc = "Field `TZSC1RST` reader - TZSC1 reset Set and reset by software"]
pub type TZSC1RST_R = crate::BitReader<bool>;
#[doc = "Field `TZSC1RST` writer - TZSC1 reset Set and reset by software"]
pub type TZSC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPDMA1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpdma1rst(&self) -> GPDMA1RST_R {
        GPDMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDMA2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn gpdma2rst(&self) -> GPDMA2RST_R {
        GPDMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC block reset Set and reset by software."]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CORDIC block reset Set and reset by software."]
    #[inline(always)]
    pub fn cordicrst(&self) -> CORDICRST_R {
        CORDICRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FMAC block reset Set and reset by software."]
    #[inline(always)]
    pub fn fmacrst(&self) -> FMACRST_R {
        FMACRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG block reset Set and reset by software."]
    #[inline(always)]
    pub fn ramcfgrst(&self) -> RAMCFGRST_R {
        RAMCFGRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - ETHRST block reset Set and reset by software"]
    #[inline(always)]
    pub fn ethrst(&self) -> ETHRST_R {
        ETHRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - TZSC1 reset Set and reset by software"]
    #[inline(always)]
    pub fn tzsc1rst(&self) -> TZSC1RST_R {
        TZSC1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma1rst(&mut self) -> GPDMA1RST_W<0> {
        GPDMA1RST_W::new(self)
    }
    #[doc = "Bit 1 - GPDMA2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma2rst(&mut self) -> GPDMA2RST_W<1> {
        GPDMA2RST_W::new(self)
    }
    #[doc = "Bit 12 - CRC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<12> {
        CRCRST_W::new(self)
    }
    #[doc = "Bit 14 - CORDIC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cordicrst(&mut self) -> CORDICRST_W<14> {
        CORDICRST_W::new(self)
    }
    #[doc = "Bit 15 - FMAC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmacrst(&mut self) -> FMACRST_W<15> {
        FMACRST_W::new(self)
    }
    #[doc = "Bit 17 - RAMCFG block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgrst(&mut self) -> RAMCFGRST_W<17> {
        RAMCFGRST_W::new(self)
    }
    #[doc = "Bit 19 - ETHRST block reset Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn ethrst(&mut self) -> ETHRST_W<19> {
        ETHRST_W::new(self)
    }
    #[doc = "Bit 24 - TZSC1 reset Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn tzsc1rst(&mut self) -> TZSC1RST_W<24> {
        TZSC1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB1 reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1rstr](index.html) module"]
pub struct AHB1RSTR_SPEC;
impl crate::RegisterSpec for AHB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1rstr::R](R) reader structure"]
impl crate::Readable for AHB1RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1rstr::W](W) writer structure"]
impl crate::Writable for AHB1RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
