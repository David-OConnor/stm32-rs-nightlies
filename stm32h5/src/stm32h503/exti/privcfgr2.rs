#[doc = "Register `PRIVCFGR2` reader"]
pub struct R(crate::R<PRIVCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVCFGR2` writer"]
pub struct W(crate::W<PRIVCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR2_SPEC>;
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
impl From<crate::W<PRIVCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIV37` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV37_R = crate::BitReader<bool>;
#[doc = "Field `PRIV37` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV37_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV38` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV38_R = crate::BitReader<bool>;
#[doc = "Field `PRIV38` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV38_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV39` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV39_R = crate::BitReader<bool>;
#[doc = "Field `PRIV39` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV39_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV40` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV40_R = crate::BitReader<bool>;
#[doc = "Field `PRIV40` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV40_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV41` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV41_R = crate::BitReader<bool>;
#[doc = "Field `PRIV41` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV41_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV42` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV42_R = crate::BitReader<bool>;
#[doc = "Field `PRIV42` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV42_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV47` reader - Privilege enable on event input x"]
pub type PRIV47_R = crate::BitReader<bool>;
#[doc = "Field `PRIV47` writer - Privilege enable on event input x"]
pub type PRIV47_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV49` reader - Privilege enable on event input x (x = 50 to 49)"]
pub type PRIV49_R = crate::BitReader<bool>;
#[doc = "Field `PRIV49` writer - Privilege enable on event input x (x = 50 to 49)"]
pub type PRIV49_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV50` reader - Privilege enable on event input x (x = 50 to 49)"]
pub type PRIV50_R = crate::BitReader<bool>;
#[doc = "Field `PRIV50` writer - Privilege enable on event input x (x = 50 to 49)"]
pub type PRIV50_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
#[doc = "Field `PRIV53` reader - Privilege enable on event input x"]
pub type PRIV53_R = crate::BitReader<bool>;
#[doc = "Field `PRIV53` writer - Privilege enable on event input x"]
pub type PRIV53_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Privilege enable on event input x"]
    #[inline(always)]
    pub fn priv47(&self) -> PRIV47_R {
        PRIV47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    pub fn priv49(&self) -> PRIV49_R {
        PRIV49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    pub fn priv50(&self) -> PRIV50_R {
        PRIV50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Privilege enable on event input x"]
    #[inline(always)]
    pub fn priv53(&self) -> PRIV53_R {
        PRIV53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv37(&mut self) -> PRIV37_W<5> {
        PRIV37_W::new(self)
    }
    #[doc = "Bit 6 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv38(&mut self) -> PRIV38_W<6> {
        PRIV38_W::new(self)
    }
    #[doc = "Bit 7 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv39(&mut self) -> PRIV39_W<7> {
        PRIV39_W::new(self)
    }
    #[doc = "Bit 8 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv40(&mut self) -> PRIV40_W<8> {
        PRIV40_W::new(self)
    }
    #[doc = "Bit 9 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv41(&mut self) -> PRIV41_W<9> {
        PRIV41_W::new(self)
    }
    #[doc = "Bit 10 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv42(&mut self) -> PRIV42_W<10> {
        PRIV42_W::new(self)
    }
    #[doc = "Bit 15 - Privilege enable on event input x"]
    #[inline(always)]
    #[must_use]
    pub fn priv47(&mut self) -> PRIV47_W<15> {
        PRIV47_W::new(self)
    }
    #[doc = "Bit 17 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    #[must_use]
    pub fn priv49(&mut self) -> PRIV49_W<17> {
        PRIV49_W::new(self)
    }
    #[doc = "Bit 18 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    #[must_use]
    pub fn priv50(&mut self) -> PRIV50_W<18> {
        PRIV50_W::new(self)
    }
    #[doc = "Bit 21 - Privilege enable on event input x"]
    #[inline(always)]
    #[must_use]
    pub fn priv53(&mut self) -> PRIV53_W<21> {
        PRIV53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI privilege configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcfgr2](index.html) module"]
pub struct PRIVCFGR2_SPEC;
impl crate::RegisterSpec for PRIVCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privcfgr2::R](R) reader structure"]
impl crate::Readable for PRIVCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privcfgr2::W](W) writer structure"]
impl crate::Writable for PRIVCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIVCFGR2 to value 0"]
impl crate::Resettable for PRIVCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
