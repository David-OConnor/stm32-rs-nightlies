#[doc = "Register `HASH_CR` reader"]
pub struct R(crate::R<HASH_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_CR` writer"]
pub struct W(crate::W<HASH_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CR_SPEC>;
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
impl From<crate::W<HASH_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` writer - INIT"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `DMAE` reader - DMAE"]
pub type DMAE_R = crate::BitReader<bool>;
#[doc = "Field `DMAE` writer - DMAE"]
pub type DMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `DATATYPE` reader - DATATYPE"]
pub type DATATYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATYPE` writer - DATATYPE"]
pub type DATATYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE` reader - MODE"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - MODE"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `ALGO0` reader - ALGO0"]
pub type ALGO0_R = crate::BitReader<bool>;
#[doc = "Field `ALGO0` writer - ALGO0"]
pub type ALGO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `NBW` reader - NBW"]
pub type NBW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DINNE` reader - DINNE"]
pub type DINNE_R = crate::BitReader<bool>;
#[doc = "Field `MDMAT` reader - MDMAT"]
pub type MDMAT_R = crate::BitReader<bool>;
#[doc = "Field `MDMAT` writer - MDMAT"]
pub type MDMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `DMAA` writer - DMAA"]
pub type DMAA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `LKEY` reader - LKEY"]
pub type LKEY_R = crate::BitReader<bool>;
#[doc = "Field `LKEY` writer - LKEY"]
pub type LKEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
#[doc = "Field `ALGO1` reader - ALGO1"]
pub type ALGO1_R = crate::BitReader<bool>;
#[doc = "Field `ALGO1` writer - ALGO1"]
pub type ALGO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - DMAE"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DATATYPE"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ALGO0"]
    #[inline(always)]
    pub fn algo0(&self) -> ALGO0_R {
        ALGO0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - NBW"]
    #[inline(always)]
    pub fn nbw(&self) -> NBW_R {
        NBW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DINNE"]
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MDMAT"]
    #[inline(always)]
    pub fn mdmat(&self) -> MDMAT_R {
        MDMAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - LKEY"]
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - ALGO1"]
    #[inline(always)]
    pub fn algo1(&self) -> ALGO1_R {
        ALGO1_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - INIT"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<2> {
        INIT_W::new(self)
    }
    #[doc = "Bit 3 - DMAE"]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DMAE_W<3> {
        DMAE_W::new(self)
    }
    #[doc = "Bits 4:5 - DATATYPE"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<4> {
        DATATYPE_W::new(self)
    }
    #[doc = "Bit 6 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<6> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - ALGO0"]
    #[inline(always)]
    #[must_use]
    pub fn algo0(&mut self) -> ALGO0_W<7> {
        ALGO0_W::new(self)
    }
    #[doc = "Bit 13 - MDMAT"]
    #[inline(always)]
    #[must_use]
    pub fn mdmat(&mut self) -> MDMAT_W<13> {
        MDMAT_W::new(self)
    }
    #[doc = "Bit 14 - DMAA"]
    #[inline(always)]
    #[must_use]
    pub fn dmaa(&mut self) -> DMAA_W<14> {
        DMAA_W::new(self)
    }
    #[doc = "Bit 16 - LKEY"]
    #[inline(always)]
    #[must_use]
    pub fn lkey(&mut self) -> LKEY_W<16> {
        LKEY_W::new(self)
    }
    #[doc = "Bit 18 - ALGO1"]
    #[inline(always)]
    #[must_use]
    pub fn algo1(&mut self) -> ALGO1_W<18> {
        ALGO1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_cr](index.html) module"]
pub struct HASH_CR_SPEC;
impl crate::RegisterSpec for HASH_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_cr::R](R) reader structure"]
impl crate::Readable for HASH_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_cr::W](W) writer structure"]
impl crate::Writable for HASH_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_CR to value 0"]
impl crate::Resettable for HASH_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
