#[doc = "Register `CR4` reader"]
pub struct R(crate::R<CR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR4` writer"]
pub struct W(crate::W<CR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR4_SPEC>;
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
impl From<crate::W<CR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WP1` reader - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP1_R = crate::BitReader<bool>;
#[doc = "Field `WP1` writer - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `WP2` reader - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP2_R = crate::BitReader<bool>;
#[doc = "Field `WP2` writer - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `WP3` reader - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP3_R = crate::BitReader<bool>;
#[doc = "Field `WP3` writer - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `WP4` reader - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP4_R = crate::BitReader<bool>;
#[doc = "Field `WP4` writer - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
#[doc = "Field `WP6` reader - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP6_R = crate::BitReader<bool>;
#[doc = "Field `WP6` writer - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    pub fn wp6(&self) -> WP6_R {
        WP6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    #[must_use]
    pub fn wp1(&mut self) -> WP1_W<0> {
        WP1_W::new(self)
    }
    #[doc = "Bit 1 - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    #[must_use]
    pub fn wp2(&mut self) -> WP2_W<1> {
        WP2_W::new(self)
    }
    #[doc = "Bit 2 - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    #[must_use]
    pub fn wp3(&mut self) -> WP3_W<2> {
        WP3_W::new(self)
    }
    #[doc = "Bit 3 - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    #[must_use]
    pub fn wp4(&mut self) -> WP4_W<3> {
        WP4_W::new(self)
    }
    #[doc = "Bit 5 - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    #[must_use]
    pub fn wp6(&mut self) -> WP6_W<5> {
        WP6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr4](index.html) module"]
pub struct CR4_SPEC;
impl crate::RegisterSpec for CR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr4::R](R) reader structure"]
impl crate::Readable for CR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr4::W](W) writer structure"]
impl crate::Writable for CR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for CR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
