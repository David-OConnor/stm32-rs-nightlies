#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Peripheral Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Peripheral Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `START` reader - Start the operation"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start the operation"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SECLVL` reader - Security Enable"]
pub type SECLVL_R = crate::BitReader<bool>;
#[doc = "Field `SECLVL` writer - Security Enable"]
pub type SECLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - PKA Operation Mode"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - PKA Operation Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 6, O>;
#[doc = "Field `PROCENDIE` reader - End of operation interrupt enable"]
pub type PROCENDIE_R = crate::BitReader<bool>;
#[doc = "Field `PROCENDIE` writer - End of operation interrupt enable"]
pub type PROCENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RAMERRIE` reader - RAM error interrupt enable"]
pub type RAMERRIE_R = crate::BitReader<bool>;
#[doc = "Field `RAMERRIE` writer - RAM error interrupt enable"]
pub type RAMERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ADDRERRIE` reader - Address error interrupt enable"]
pub type ADDRERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ADDRERRIE` writer - Address error interrupt enable"]
pub type ADDRERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Peripheral Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start the operation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security Enable"]
    #[inline(always)]
    pub fn seclvl(&self) -> SECLVL_R {
        SECLVL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PKA Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Start the operation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<1> {
        START_W::new(self)
    }
    #[doc = "Bit 2 - Security Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seclvl(&mut self) -> SECLVL_W<2> {
        SECLVL_W::new(self)
    }
    #[doc = "Bits 8:13 - PKA Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Bit 17 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn procendie(&mut self) -> PROCENDIE_W<17> {
        PROCENDIE_W::new(self)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrie(&mut self) -> RAMERRIE_W<19> {
        RAMERRIE_W::new(self)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W<20> {
        ADDRERRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
