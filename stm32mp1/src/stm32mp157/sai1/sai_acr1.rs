#[doc = "Register `SAI_ACR1` reader"]
pub struct R(crate::R<SAI_ACR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_ACR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_ACR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_ACR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_ACR1` writer"]
pub struct W(crate::W<SAI_ACR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_ACR1_SPEC>;
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
impl From<crate::W<SAI_ACR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_ACR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - MODE"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - MODE"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRTCFG` reader - PRTCFG"]
pub type PRTCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRTCFG` writer - PRTCFG"]
pub type PRTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `DS` reader - DS"]
pub type DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS` writer - DS"]
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `LSBFIRST` reader - LSBFIRST"]
pub type LSBFIRST_R = crate::BitReader<bool>;
#[doc = "Field `LSBFIRST` writer - LSBFIRST"]
pub type LSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `CKSTR` reader - CKSTR"]
pub type CKSTR_R = crate::BitReader<bool>;
#[doc = "Field `CKSTR` writer - CKSTR"]
pub type CKSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `SYNCEN` reader - SYNCEN"]
pub type SYNCEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCEN` writer - SYNCEN"]
pub type SYNCEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `MONO` reader - MONO"]
pub type MONO_R = crate::BitReader<bool>;
#[doc = "Field `MONO` writer - MONO"]
pub type MONO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `OUTDRIV` reader - OUTDRIV"]
pub type OUTDRIV_R = crate::BitReader<bool>;
#[doc = "Field `OUTDRIV` writer - OUTDRIV"]
pub type OUTDRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `SAIEN` reader - SAIEN"]
pub type SAIEN_R = crate::BitReader<bool>;
#[doc = "Field `SAIEN` writer - SAIEN"]
pub type SAIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `NODIV` reader - NODIV"]
pub type NODIV_R = crate::BitReader<bool>;
#[doc = "Field `NODIV` writer - NODIV"]
pub type NODIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `MCKDIV` reader - MCKDIV"]
pub type MCKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCKDIV` writer - MCKDIV"]
pub type MCKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR1_SPEC, u8, u8, 6, O>;
#[doc = "Field `OSR` reader - OSR"]
pub type OSR_R = crate::BitReader<bool>;
#[doc = "Field `OSR` writer - OSR"]
pub type OSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
#[doc = "Field `MCKEN` reader - MCKEN"]
pub type MCKEN_R = crate::BitReader<bool>;
#[doc = "Field `MCKEN` writer - MCKEN"]
pub type MCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PRTCFG"]
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:7 - DS"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - LSBFIRST"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CKSTR"]
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - SYNCEN"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - MONO"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OUTDRIV"]
    #[inline(always)]
    pub fn outdriv(&self) -> OUTDRIV_R {
        OUTDRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - SAIEN"]
    #[inline(always)]
    pub fn saien(&self) -> SAIEN_R {
        SAIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - NODIV"]
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:25 - MCKDIV"]
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - OSR"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MCKEN"]
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - PRTCFG"]
    #[inline(always)]
    #[must_use]
    pub fn prtcfg(&mut self) -> PRTCFG_W<2> {
        PRTCFG_W::new(self)
    }
    #[doc = "Bits 5:7 - DS"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<5> {
        DS_W::new(self)
    }
    #[doc = "Bit 8 - LSBFIRST"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<8> {
        LSBFIRST_W::new(self)
    }
    #[doc = "Bit 9 - CKSTR"]
    #[inline(always)]
    #[must_use]
    pub fn ckstr(&mut self) -> CKSTR_W<9> {
        CKSTR_W::new(self)
    }
    #[doc = "Bits 10:11 - SYNCEN"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<10> {
        SYNCEN_W::new(self)
    }
    #[doc = "Bit 12 - MONO"]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<12> {
        MONO_W::new(self)
    }
    #[doc = "Bit 13 - OUTDRIV"]
    #[inline(always)]
    #[must_use]
    pub fn outdriv(&mut self) -> OUTDRIV_W<13> {
        OUTDRIV_W::new(self)
    }
    #[doc = "Bit 16 - SAIEN"]
    #[inline(always)]
    #[must_use]
    pub fn saien(&mut self) -> SAIEN_W<16> {
        SAIEN_W::new(self)
    }
    #[doc = "Bit 17 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<17> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 19 - NODIV"]
    #[inline(always)]
    #[must_use]
    pub fn nodiv(&mut self) -> NODIV_W<19> {
        NODIV_W::new(self)
    }
    #[doc = "Bits 20:25 - MCKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn mckdiv(&mut self) -> MCKDIV_W<20> {
        MCKDIV_W::new(self)
    }
    #[doc = "Bit 26 - OSR"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OSR_W<26> {
        OSR_W::new(self)
    }
    #[doc = "Bit 27 - MCKEN"]
    #[inline(always)]
    #[must_use]
    pub fn mcken(&mut self) -> MCKEN_W<27> {
        MCKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_acr1](index.html) module"]
pub struct SAI_ACR1_SPEC;
impl crate::RegisterSpec for SAI_ACR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_acr1::R](R) reader structure"]
impl crate::Readable for SAI_ACR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_acr1::W](W) writer structure"]
impl crate::Writable for SAI_ACR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_ACR1 to value 0x40"]
impl crate::Resettable for SAI_ACR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
