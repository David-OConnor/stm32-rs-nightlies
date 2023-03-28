#[doc = "Register `C2CR` reader"]
pub struct R(crate::R<C2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2CR` writer"]
pub struct W(crate::W<C2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR_SPEC>;
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
impl From<crate::W<C2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PG` reader - Programming"]
pub type PG_R = crate::BitReader<PG_A>;
#[doc = "Programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PG_A {
    #[doc = "0: Flash programming disabled"]
    Disabled = 0,
    #[doc = "1: Flash programming enabled"]
    Enabled = 1,
}
impl From<PG_A> for bool {
    #[inline(always)]
    fn from(variant: PG_A) -> Self {
        variant as u8 != 0
    }
}
impl PG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PG_A {
        match self.bits {
            false => PG_A::Disabled,
            true => PG_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PG_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PG_A::Enabled
    }
}
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR_SPEC, PG_A, O>;
impl<'a, const O: u8> PG_W<'a, O> {
    #[doc = "Flash programming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PG_A::Disabled)
    }
    #[doc = "Flash programming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PG_A::Enabled)
    }
}
#[doc = "Field `PER` reader - Page erase"]
pub type PER_R = crate::BitReader<PER_A>;
#[doc = "Page erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    #[doc = "0: Page erase disabled"]
    Disabled = 0,
    #[doc = "1: Page erase enabled"]
    Enabled = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::Disabled,
            true => PER_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PER_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PER_A::Enabled
    }
}
#[doc = "Field `PER` writer - Page erase"]
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR_SPEC, PER_A, O>;
impl<'a, const O: u8> PER_W<'a, O> {
    #[doc = "Page erase disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PER_A::Disabled)
    }
    #[doc = "Page erase enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PER_A::Enabled)
    }
}
#[doc = "Field `MER` reader - Mass erase"]
pub type MER_R = crate::BitReader<MER_A>;
#[doc = "Mass erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER_A {
    #[doc = "0: No mass erase"]
    NoErase = 0,
    #[doc = "1: Trigger mass erase"]
    MassErase = 1,
}
impl From<MER_A> for bool {
    #[inline(always)]
    fn from(variant: MER_A) -> Self {
        variant as u8 != 0
    }
}
impl MER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MER_A {
        match self.bits {
            false => MER_A::NoErase,
            true => MER_A::MassErase,
        }
    }
    #[doc = "Checks if the value of the field is `NoErase`"]
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        *self == MER_A::NoErase
    }
    #[doc = "Checks if the value of the field is `MassErase`"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER_A::MassErase
    }
}
#[doc = "Field `MER` writer - Mass erase"]
pub type MER_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR_SPEC, MER_A, O>;
impl<'a, const O: u8> MER_W<'a, O> {
    #[doc = "No mass erase"]
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut W {
        self.variant(MER_A::NoErase)
    }
    #[doc = "Trigger mass erase"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER_A::MassErase)
    }
}
#[doc = "Field `PNB` reader - Page number selection"]
pub type PNB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PNB` writer - Page number selection"]
pub type PNB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, C2CR_SPEC, u8, u8, 7, O>;
#[doc = "Field `STRT` reader - Start"]
pub type STRT_R = crate::BitReader<STRTR_A>;
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTR_A {
    #[doc = "0: Options modification completed or idle"]
    Done = 0,
}
impl From<STRTR_A> for bool {
    #[inline(always)]
    fn from(variant: STRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl STRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STRTR_A> {
        match self.bits {
            false => Some(STRTR_A::Done),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Done`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STRTR_A::Done
    }
}
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTW_AW {
    #[doc = "1: Trigger options programming operation"]
    Start = 1,
}
impl From<STRTW_AW> for bool {
    #[inline(always)]
    fn from(variant: STRTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT` writer - Start"]
pub type STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR_SPEC, STRTW_AW, O>;
impl<'a, const O: u8> STRT_W<'a, O> {
    #[doc = "Trigger options programming operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STRTW_AW::Start)
    }
}
#[doc = "Field `FSTPG` reader - Fast programming"]
pub type FSTPG_R = crate::BitReader<FSTPG_A>;
#[doc = "Fast programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSTPG_A {
    #[doc = "0: Fast programming disabled"]
    Disabled = 0,
    #[doc = "1: Fast programming enabled"]
    Enabled = 1,
}
impl From<FSTPG_A> for bool {
    #[inline(always)]
    fn from(variant: FSTPG_A) -> Self {
        variant as u8 != 0
    }
}
impl FSTPG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSTPG_A {
        match self.bits {
            false => FSTPG_A::Disabled,
            true => FSTPG_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSTPG_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSTPG_A::Enabled
    }
}
#[doc = "Field `FSTPG` writer - Fast programming"]
pub type FSTPG_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR_SPEC, FSTPG_A, O>;
impl<'a, const O: u8> FSTPG_W<'a, O> {
    #[doc = "Fast programming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FSTPG_A::Disabled)
    }
    #[doc = "Fast programming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FSTPG_A::Enabled)
    }
}
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EOPIE_R = crate::BitReader<EOPIE_A>;
#[doc = "End of operation interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPIE_A {
    #[doc = "0: End of program interrupt disable"]
    Disabled = 0,
    #[doc = "1: End of program interrupt enable"]
    Enabled = 1,
}
impl From<EOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOPIE_A {
        match self.bits {
            false => EOPIE_A::Disabled,
            true => EOPIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIE_A::Enabled
    }
}
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR_SPEC, EOPIE_A, O>;
impl<'a, const O: u8> EOPIE_W<'a, O> {
    #[doc = "End of program interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOPIE_A::Disabled)
    }
    #[doc = "End of program interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOPIE_A::Enabled)
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: OPERR Error interrupt disable"]
    Disabled = 0,
    #[doc = "1: OPERR Error interrupt enable"]
    Enabled = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::Disabled,
            true => ERRIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR_SPEC, ERRIE_A, O>;
impl<'a, const O: u8> ERRIE_W<'a, O> {
    #[doc = "OPERR Error interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Disabled)
    }
    #[doc = "OPERR Error interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::Enabled)
    }
}
#[doc = "Field `RDERRIE` reader - RDERRIE"]
pub type RDERRIE_R = crate::BitReader<RDERRIE_A>;
#[doc = "RDERRIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRIE_A {
    #[doc = "0: PCROP read error interrupt disable"]
    Disabled = 0,
    #[doc = "1: PCROP read error interrupt enable"]
    Enabled = 1,
}
impl From<RDERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDERRIE_A {
        match self.bits {
            false => RDERRIE_A::Disabled,
            true => RDERRIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDERRIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDERRIE_A::Enabled
    }
}
#[doc = "Field `RDERRIE` writer - RDERRIE"]
pub type RDERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR_SPEC, RDERRIE_A, O>;
impl<'a, const O: u8> RDERRIE_W<'a, O> {
    #[doc = "PCROP read error interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDERRIE_A::Disabled)
    }
    #[doc = "PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDERRIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass erase"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - Page number selection"]
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RDERRIE"]
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<0> {
        PG_W::new(self)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<1> {
        PER_W::new(self)
    }
    #[doc = "Bit 2 - Mass erase"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<2> {
        MER_W::new(self)
    }
    #[doc = "Bits 3:9 - Page number selection"]
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PNB_W<3> {
        PNB_W::new(self)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<16> {
        STRT_W::new(self)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    #[must_use]
    pub fn fstpg(&mut self) -> FSTPG_W<18> {
        FSTPG_W::new(self)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<24> {
        EOPIE_W::new(self)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<25> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 26 - RDERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rderrie(&mut self) -> RDERRIE_W<26> {
        RDERRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash CPU2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr](index.html) module"]
pub struct C2CR_SPEC;
impl crate::RegisterSpec for C2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2cr::R](R) reader structure"]
impl crate::Readable for C2CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2cr::W](W) writer structure"]
impl crate::Writable for C2CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2CR to value 0xc000_0000"]
impl crate::Resettable for C2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
